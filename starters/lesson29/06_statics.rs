// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 29-6 · The Shared Altar                          175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A `static` is one value for the whole program. A `static mut` can be
//   changed, but every access is unsafe: two threads writing at once is a
//   data race, which is undefined behavior.
//
//   Edition 2024 goes further and REFUSES references to a static mut
//   (`LOG.push(..)` takes `&mut LOG`). You almost never need one. Safe
//   tools do the job:
//
//     use std::sync::atomic::{AtomicU32, Ordering};
//     static VISITS: AtomicU32 = AtomicU32::new(0);
//     VISITS.fetch_add(1, Ordering::Relaxed);           // a counter
//
//     use std::sync::Mutex;
//     static LOG: Mutex<Vec<String>> = Mutex::new(Vec::new());
//     LOG.lock().unwrap().push("hi".into());            // anything else
//
//     use std::sync::LazyLock;                          // built on first use
//     static NAMES: LazyLock<HashMap<u32, &str>> = LazyLock::new(|| ...);
//
//   `fetch_add` returns the OLD value. `Ordering::Relaxed` is fine for a
//   plain counter; stronger orderings matter when one value guards others.
//
// 🎯 YOUR TASK
//
//   Replace both `static mut`s with safe statics and remove all unsafe.
//   `visit` returns the visit number (1 for the first visit).

static mut VISITS: u32 = 0;
static mut LOG: Vec<String> = Vec::new();

pub fn visit() -> u32 {
    unsafe {
        VISITS += 1;
        VISITS
    }
}

pub fn log(line: &str) {
    unsafe { LOG.push(line.to_string()) }
}

pub fn log_len() -> usize {
    unsafe { LOG.len() }
}

fn main() {
    visit();
    log("hello");
    println!("{} visits, {} log lines", visit(), log_len());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn many_visitors_at_once() {
        let handles: Vec<_> = (0..8)
            .map(|_| {
                thread::spawn(|| {
                    for _ in 0..1000 {
                        visit();
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(visit(), 8001);
    }

    #[test]
    fn many_scribes() {
        thread::scope(|s| {
            for i in 0..4 {
                s.spawn(move || {
                    for j in 0..100 {
                        log(&format!("scribe {i} line {j}"));
                    }
                });
            }
        });
        assert_eq!(log_len(), 400);
    }
}
