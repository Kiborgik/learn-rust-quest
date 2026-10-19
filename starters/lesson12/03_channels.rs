// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 12-3 · Message Bees                              150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   "Don't communicate by sharing memory; share memory by communicating."
//   A CHANNEL sends values from one thread to another:
//
//     use std::sync::mpsc;               // multi-producer, single-consumer
//
//     let (tx, rx) = mpsc::channel();    // transmitter, receiver
//
//     thread::spawn(move || {
//         tx.send(42).unwrap();          // send MOVES the value across
//     });
//
//     let got = rx.recv().unwrap();      // blocks until a value arrives
//
//   Multiple producers: clone the sender.
//
//     let tx2 = tx.clone();
//
//   Receiving everything:
//
//     for msg in rx { ... }              // ends when ALL senders are dropped
//
//   ⚠️ That last rule matters: if ANY sender is still alive (for example
//   the original `tx` held by main), `for msg in rx` waits forever.
//   Drop it explicitly: `drop(tx);`
//
// 🎯 YOUR TASK
//
//   `gather_nectar(flowers)`: spawn one bee thread per flower. Each bee
//   sends back the flower's nectar = (its length * 10).
//   Collect all messages and return their SUM.
//
//   The skeleton is there, but it doesn't compile yet. When it compiles,
//   make sure the receiving loop can finish!

use std::sync::mpsc;
use std::thread;

fn gather_nectar(flowers: Vec<String>) -> usize {
    let (tx, rx) = mpsc::channel();

    for flower in flowers {
        thread::spawn(move || {
            tx.send(flower.len() * 10).unwrap();
        });
    }

    // TODO: make sure the loop below can end

    let mut total = 0;
    for nectar in rx {
        total += nectar;
    }
    total
}

fn main() {
    let flowers = vec![String::from("rose"), String::from("lily")];
    println!("{}", gather_nectar(flowers));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nectar() {
        let flowers = vec![String::from("rose"), String::from("lily"), String::from("daisy")];
        assert_eq!(gather_nectar(flowers), 130);
        assert_eq!(gather_nectar(Vec::new()), 0);
    }
}
