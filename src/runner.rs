//! Compiles and runs one exercise, so a broken exercise never stops the game.
//! Single-file exercises use `rustc` directly; workshop exercises are Cargo
//! packages and use `cargo`.

use crate::exercises::{Check, Exercise};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// Rust edition used for exercises (also written into exercises/Cargo.toml).
pub const EDITION: &str = "2024";

/// Exercises (especially the threading ones) can hang. Kill them after this.
const RUN_TIMEOUT: Duration = Duration::from_secs(10);
/// Cargo builds can be slow (first build downloads and compiles crates).
const BUILD_TIMEOUT: Duration = Duration::from_secs(600);

struct RunOutput {
    status: ExitStatus,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

/// Like `Command::output`, but gives up after `timeout`.
/// Returns `Ok(None)` on timeout.
fn output_with_timeout(cmd: &mut Command, timeout: Duration) -> std::io::Result<Option<RunOutput>> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Drain the pipes on background threads so a chatty child can't block.
    let mut out_pipe = child.stdout.take().expect("piped stdout");
    let mut err_pipe = child.stderr.take().expect("piped stderr");
    let out_reader = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = out_pipe.read_to_end(&mut buf);
        buf
    });
    let err_reader = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = err_pipe.read_to_end(&mut buf);
        buf
    });

    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }
        thread::sleep(Duration::from_millis(20));
    };

    Ok(Some(RunOutput {
        status,
        stdout: out_reader.join().unwrap_or_default(),
        stderr: err_reader.join().unwrap_or_default(),
    }))
}

pub enum Outcome {
    /// Compiled and behaved correctly. Contains program/test output and any warnings.
    Pass { output: String, warnings: String },
    /// rustc rejected the code.
    CompileError(String),
    /// Compiled, but wrong output / failing tests / panic.
    Fail(String),
}

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rustc() -> String {
    std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into())
}

/// Checks the learner's working copy of an exercise.
pub fn check(ex: &Exercise) -> Outcome {
    check_file(ex, &root().join(ex.path))
}

/// Checks `src` against `ex`'s rules (used for working copies and starters).
/// For Cargo exercises `src` is any file inside the package.
pub fn check_file(ex: &Exercise, src: &Path) -> Outcome {
    if ex.check.is_cargo() {
        return check_cargo(ex, &package_dir(src));
    }
    let out_dir = root().join("target").join("quest");
    let _ = std::fs::create_dir_all(&out_dir);
    let bin = out_dir.join(format!("{}{}", ex.id, std::env::consts::EXE_SUFFIX));

    // Compile from the project root with a relative path, so errors and
    // panics show `exercises/lesson01/...` instead of a long absolute path.
    let mut cmd = Command::new(rustc());
    cmd.current_dir(root())
        .arg(format!("--edition={EDITION}"))
        .args(["--color=always", "-o"])
        .arg(&bin)
        .arg(src.strip_prefix(root()).unwrap_or(src));
    if let Check::Test = ex.check {
        // `main` is unused in test builds; don't nag about it.
        cmd.args(["--test", "-Adead_code"]);
    }

    let compiled = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            return Outcome::CompileError(format!(
                "Could not start `rustc`: {e}\nIs Rust installed and on your PATH?"
            ));
        }
    };
    let warnings = String::from_utf8_lossy(&compiled.stderr).into_owned();
    if !compiled.status.success() {
        return Outcome::CompileError(warnings);
    }

    let mut run = Command::new(&bin);
    if let Check::Test = ex.check {
        run.args(["--color=always", "--test-threads=1"]);
    }
    run_and_judge(ex, &mut run, warnings)
}

/// The folder of the Cargo package that contains `src`.
fn package_dir(src: &Path) -> PathBuf {
    src.ancestors()
        .skip(1)
        .find(|dir| dir.join("Cargo.toml").is_file())
        .unwrap_or_else(|| src.parent().unwrap_or(src))
        .to_path_buf()
}

fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".into())
}

fn check_cargo(ex: &Exercise, pkg: &Path) -> Outcome {
    let test = matches!(ex.check, Check::CargoTest);
    let cargo_cmd = |sub: &str| {
        let mut c = Command::new(cargo());
        c.arg(sub)
            .arg("--manifest-path")
            .arg(pkg.join("Cargo.toml"))
            .args(["--quiet", "--color=always"]);
        // The player's packages share the exercises/ workspace. Anything else
        // (starter copies checked by the tests) shares one target dir.
        if !pkg.starts_with(root().join("exercises")) {
            c.arg("--target-dir")
                .arg(root().join("target").join("starters"));
        }
        c
    };

    // Build first with a generous timeout; only the run counts against the
    // normal limit.
    let mut build = if test {
        let mut c = cargo_cmd("test");
        c.arg("--no-run");
        c
    } else {
        cargo_cmd("build")
    };
    let built = match output_with_timeout(&mut build, BUILD_TIMEOUT) {
        Ok(Some(o)) => o,
        Ok(None) => {
            return Outcome::CompileError(format!(
                "The build took longer than {} seconds and was stopped.",
                BUILD_TIMEOUT.as_secs()
            ));
        }
        Err(e) => {
            return Outcome::CompileError(format!(
                "Could not start `cargo`: {e}\nIs Rust installed and on your PATH?"
            ));
        }
    };
    let warnings = String::from_utf8_lossy(&built.stderr).into_owned();
    if !built.status.success() {
        return Outcome::CompileError(warnings);
    }

    let mut run = if test {
        let mut c = cargo_cmd("test");
        c.args(["--", "--color=always", "--test-threads=1"]);
        c
    } else {
        cargo_cmd("run")
    };
    run_and_judge(ex, &mut run, warnings)
}

/// Runs the built program (or test binary) and turns the result into a verdict.
fn run_and_judge(ex: &Exercise, run: &mut Command, warnings: String) -> Outcome {
    // Cargo adds a little startup overhead on top of the program itself.
    let timeout = if ex.check.is_cargo() {
        RUN_TIMEOUT * 2
    } else {
        RUN_TIMEOUT
    };
    let ran = match output_with_timeout(run, timeout) {
        Ok(Some(o)) => o,
        Ok(None) => {
            return Outcome::Fail(format!(
                "The program was still running after {} seconds, so it was stopped.\n\
                 Is something waiting forever? (an endless loop, a channel whose\n\
                 sender is never dropped, a lock that is never released...)",
                timeout.as_secs()
            ));
        }
        Err(e) => return Outcome::Fail(format!("Could not run the compiled program: {e}")),
    };
    let stdout = String::from_utf8_lossy(&ran.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&ran.stderr).into_owned();

    match ex.check {
        Check::Test | Check::CargoTest => {
            if ran.status.success() {
                Outcome::Pass {
                    output: stdout,
                    warnings,
                }
            } else {
                Outcome::Fail(format!("{stdout}{stderr}"))
            }
        }
        Check::Output(expected) | Check::CargoRun(expected) => {
            if !ran.status.success() {
                return Outcome::Fail(format!(
                    "{stdout}{stderr}\nThe program crashed (panicked) while running."
                ));
            }
            if normalize(&stdout) == normalize(expected) {
                Outcome::Pass {
                    output: stdout,
                    warnings,
                }
            } else {
                Outcome::Fail(output_diff(expected, &stdout))
            }
        }
    }
}

/// (passed, failed) summed over every `test result:` line in test output
/// (one line per test binary), if there are any tests.
pub fn test_counts(output: &str) -> Option<(u32, u32)> {
    let mut found = None;
    for line in crate::ui::strip_ansi(output).lines() {
        let Some((_, rest)) = line.split_once("test result:") else {
            continue;
        };
        let count = |word: &str| {
            rest.split(';')
                .find_map(|part| {
                    let num = part.trim().strip_suffix(word)?.split(' ').next_back()?;
                    num.parse::<u32>().ok()
                })
                .unwrap_or(0)
        };
        let (p, f) = found.unwrap_or((0, 0));
        found = Some((p + count(" passed"), f + count(" failed")));
    }
    found.filter(|&(p, f)| p + f > 0)
}

fn normalize(s: &str) -> String {
    s.replace("\r\n", "\n")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn output_diff(expected: &str, got: &str) -> String {
    use crate::ui::{dim, green, red};
    let expected = normalize(expected);
    let got = normalize(got);
    let exp: Vec<&str> = expected.lines().collect();
    let act: Vec<&str> = got.lines().collect();

    let mut out = String::from("The program ran, but its output isn't quite right:\n\n");
    for i in 0..exp.len().max(act.len()) {
        let e = exp.get(i).copied();
        let a = act.get(i).copied();
        if e == a {
            out.push_str(&format!("  {}\n", dim(format!("  {}", e.unwrap_or("")))));
        } else {
            if let Some(e) = e {
                out.push_str(&format!("  {}\n", green(format!("+ {e}"))));
            }
            if let Some(a) = a {
                out.push_str(&format!("  {}\n", red(format!("- {a}"))));
            }
        }
    }
    out.push_str(&format!(
        "\n  {}  {}\n",
        green("+ expected"),
        red("- your program printed")
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_tests_across_binaries() {
        let out = "running 3 tests\n\
            test result: \x1b[31mFAILED\x1b[0m. 1 passed; 2 failed; 0 ignored\n\
            test result: ok. 4 passed; 0 failed; 0 ignored; finished in 0.01s\n";
        assert_eq!(test_counts(out), Some((5, 2)));
        assert_eq!(test_counts("error[E0308]: mismatched types"), None);
        assert_eq!(test_counts("test result: ok. 0 passed; 0 failed;"), None);
    }
}
