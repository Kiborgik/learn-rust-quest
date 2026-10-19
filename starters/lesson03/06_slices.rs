// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 3-6 · Slicing the Map                            100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A SLICE is a borrowed view into part of a collection: no copying.
//
//     let s = String::from("hello world");
//     let hello: &str = &s[0..5];     // bytes 0..5  → "hello"
//     let world: &str = &s[6..];      // from 6 to the end → "world"
//     let all:   &str = &s[..];       // everything
//
//     let loot = [10, 20, 30, 40];
//     let mid: &[i32] = &loot[1..3];  // [20, 30]
//     mid.len();                      // 2
//
//   • `&str` is literally a slice of text; `&[T]` is a slice of Ts.
//   • A function taking `&[T]` accepts arrays, Vecs and slices alike.
//   • ⚠️ String slice indexes are BYTE positions. Slicing through the
//     middle of a multi-byte character (like 'é' or '🦀') panics.
//
//   Because a slice BORROWS its source, the source can't change while
//   the slice is in use:
//
//     let mut s = String::from("hello world");
//     let word = &s[0..5];
//     s.clear();               // ✘ ERROR: s is borrowed by `word`
//     println!("{word}");
//
//   Looping over bytes: `s.as_bytes()` gives a `&[u8]`, and `b' '` is
//   the byte value of a space.
//
// 🎯 YOUR TASK
//
//   1. `first_word` has a bug: the result includes the space.
//   2. Write `middle`: everything except the first and last element.
//   3. Write `sum`: add up all numbers in a slice.
//   4. `keep_first_word` has a borrowing error. Fix it so it returns
//      an owned `String` of the first word (`.to_string()` helps).

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b' ' {
            return &s[0..i + 1];
        }
    }
    &s[..]
}

fn middle(items: &[i32]) -> &[i32] {
    todo!()
}

fn sum(numbers: &[u32]) -> u32 {
    todo!()
}

fn keep_first_word() -> String {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    word.to_string()
}

fn main() {
    println!("[{}]", first_word("treasure map"));
    println!("{:?}", middle(&[1, 2, 3, 4, 5]));
    println!("{}", sum(&[1, 2, 3]));
    println!("{}", keep_first_word());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(first_word("treasure map"), "treasure");
        assert_eq!(first_word("X"), "X");
    }

    #[test]
    fn middles() {
        assert_eq!(middle(&[1, 2, 3, 4, 5]), &[2, 3, 4]);
        assert_eq!(middle(&[7, 8]), &[] as &[i32]);
    }

    #[test]
    fn sums() {
        let loot = [10, 20, 30, 40];
        assert_eq!(sum(&loot), 100);
        assert_eq!(sum(&loot[1..3]), 50);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn keeping() {
        assert_eq!(keep_first_word(), "hello");
    }
}
