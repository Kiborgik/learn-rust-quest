//! Which starter version each exercise file is, so `cargo run` after a
//! `git pull` can update every file the player never touched.
//!
//! Two sources:
//! - `src/starter_hashes.txt` (in git, built into the game): every
//!   published version of every starter. A file matching one of them is
//!   untouched. A test keeps this list complete.
//! - `.quest/starters.txt` (per player): the version each file was last
//!   copied from, so an edited quest can say "an update exists".
//!
//! Lines are `hash path`. The hash is FNV-1a over the file with `\r`
//! removed: stable across Rust versions and line-ending styles.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[cfg(test)]
pub const HISTORY_FILE: &str = "src/starter_hashes.txt";
const HISTORY: &str = include_str!("starter_hashes.txt");

/// FNV-1a, 64 bit. Small, fast and the same on every platform and release.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| {
        (h ^ b as u64).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Hash of a file's content, ignoring line-ending style.
pub fn content_hash(data: &[u8]) -> u64 {
    let no_cr: Vec<u8> = data.iter().copied().filter(|&b| b != b'\r').collect();
    fnv1a(&no_cr)
}

/// `lesson02/01_if_else.rs`, with forward slashes on every OS.
pub fn key(rel: &Path) -> String {
    rel.to_string_lossy().replace('\\', "/")
}

fn parse(text: &str) -> impl Iterator<Item = (String, u64)> + '_ {
    text.lines().filter_map(|line| {
        let (hash, path) = line.split_once(' ')?;
        Some((path.to_string(), u64::from_str_radix(hash, 16).ok()?))
    })
}

static KNOWN: LazyLock<HashSet<(String, u64)>> = LazyLock::new(|| parse(HISTORY).collect());

/// Was this content ever shipped as the starter for this file?
pub fn is_known_starter(rel: &Path, hash: u64) -> bool {
    KNOWN.contains(&(key(rel), hash))
}

/// Has this file's starter ever changed after it was first shipped?
pub fn was_revised(rel: &Path) -> bool {
    let key = key(rel);
    KNOWN.iter().filter(|(p, _)| *p == key).nth(1).is_some()
}

/// The line to add to `HISTORY_FILE` for a starter's current content.
#[cfg(test)]
pub fn history_line(rel: &Path, data: &[u8]) -> String {
    format!("{:016x} {}", content_hash(data), key(rel))
}

/// Per player: the starter version each file was last copied from.
#[derive(Default)]
pub struct Versions {
    map: BTreeMap<String, u64>,
}

fn player_file() -> PathBuf {
    crate::runner::root().join(".quest").join("starters.txt")
}

impl Versions {
    pub fn load() -> Versions {
        let text = fs::read_to_string(player_file()).unwrap_or_default();
        Versions {
            map: parse(&text).collect(),
        }
    }

    pub fn get(&self, rel: &Path) -> Option<u64> {
        self.map.get(&key(rel)).copied()
    }

    /// Records that this file now matches `starter`.
    pub fn set(&mut self, rel: &Path, starter: &[u8]) {
        self.map.insert(key(rel), content_hash(starter));
    }

    pub fn save(&self) -> io::Result<()> {
        let out: String = self
            .map
            .iter()
            .map(|(p, h)| format!("{h:016x} {p}\n"))
            .collect();
        let path = player_file();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_ignores_line_endings() {
        assert_eq!(content_hash(b"a\r\nb\r\n"), content_hash(b"a\nb\n"));
        assert_ne!(content_hash(b"a\nb\n"), content_hash(b"a\nc\n"));
        // Known FNV-1a value, so the file format never changes by accident.
        assert_eq!(fnv1a(b"a"), 0xaf63_dc4c_8601_ec8c);
    }
}
