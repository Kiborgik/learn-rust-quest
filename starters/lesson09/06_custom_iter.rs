// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 9-6 · Build Your Own Stream                      150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Implement ONE method, `next`, and you get EVERY adapter and consumer
//   (map, filter, sum, take, zip...) for free, thanks to default methods:
//
//     struct Countdown { n: u32 }
//
//     impl Iterator for Countdown {
//         type Item = u32;                    // what the iterator yields
//
//         fn next(&mut self) -> Option<u32> {
//             if self.n == 0 {
//                 None                        // finished
//             } else {
//                 self.n -= 1;
//                 Some(self.n + 1)
//             }
//         }
//     }
//
//     let v: Vec<u32> = Countdown { n: 3 }.collect();          // [3, 2, 1]
//     let s: u32 = Countdown { n: 4 }.filter(|x| x % 2 == 0).sum(); // 6
//     for x in Countdown { n: 2 } { }                          // works in for
//
//   `type Item` is an "associated type": each iterator picks its own.
//   Iterators can even be INFINITE, as long as you only `take` what you need.
//
// 🎯 YOUR TASK
//
//   1. Finish `Iterator for Fibonacci` (infinite: 0, 1, 1, 2, 3, 5, 8, ...).
//   2. Fix `Iterator for Steps`: yields start, start+step, ... while < end.
//      It is missing something the trait requires.
//   3. Implement `even_fibs_below(limit)`: the sum of even Fibonacci
//      numbers below `limit`, using your iterator and adapters.
//      Tip: `.take_while(|&x| x < limit)` stops at the first false.

struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

struct Steps {
    pos: u32,
    end: u32,
    step: u32,
}

impl Iterator for Steps {
    fn next(&mut self) -> Option<u32> {
        if self.pos >= self.end {
            return None;
        }
        let value = self.pos;
        self.pos += self.step;
        Some(value)
    }
}

fn even_fibs_below(limit: u64) -> u64 {
    todo!()
}

fn main() {
    let first: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", first);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci() {
        let v: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(v, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn steps() {
        let v: Vec<u32> = Steps { pos: 2, end: 11, step: 3 }.collect();
        assert_eq!(v, vec![2, 5, 8]);
        assert_eq!(Steps { pos: 0, end: 10, step: 5 }.map(|x| x * 2).sum::<u32>(), 10);
    }

    #[test]
    fn even_fibs() {
        assert_eq!(even_fibs_below(10), 10); // 0 + 2 + 8
        assert_eq!(even_fibs_below(4_000_000), 4_613_732);
    }
}
