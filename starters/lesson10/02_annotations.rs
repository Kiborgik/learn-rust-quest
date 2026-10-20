// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 10-2 · Marking Time                              150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   When a function takes several references and returns one, the
//   compiler can't guess WHICH input the output borrows from:
//
//     fn longer(a: &str, b: &str) -> &str { .. }   // ✘ missing lifetime specifier
//
//   You tell it with a lifetime parameter, written `'a`:
//
//     fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
//         if a.len() >= b.len() { a } else { b }
//     }
//
//   Read it as: "the result is valid as long as BOTH inputs are valid".
//
//   An input that the result never borrows from needs no `'a`:
//
//     fn first_of<'a>(a: &'a str, _b: &str) -> &'a str { a }
//
//   Lifetimes DESCRIBE relationships; they never make anything live
//   longer. For callers this means:
//
//     let a = String::from("long string");
//     let result;
//     {
//         let b = String::from("xyz");
//         result = longer(&a, &b);
//     }                                  // b dropped...
//     println!("{result}");              // ✘ ...result may borrow b
//
// 🎯 YOUR TASK
//
//   1. Add lifetime annotations to `stronger` and `first_rune`.
//   2. `pick` always returns its FIRST argument, so it only needs a
//      lifetime on that one. Annotate it so the test `pick_outlives`
//      compiles (the second argument dies early there!).

fn stronger(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}

/// The first word of `spell` that starts with `rune`, or "" if none.
fn first_rune(spell: &str, rune: &str) -> &str {
    for word in spell.split_whitespace() {
        if word.starts_with(rune) {
            return word;
        }
    }
    ""
}

fn pick<'a>(keep: &'a str, ignore: &'a str) -> &'a str {
    println!("ignoring {ignore}");
    keep
}

fn main() {
    println!("{}", stronger("fire", "ice"));
    println!("{}", first_rune("old mighty magic", "mi"));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strength() {
        assert_eq!(stronger("fire", "ice"), "fire");
        assert_eq!(stronger("ice", "storm"), "storm");
    }

    #[test]
    fn runes() {
        assert_eq!(first_rune("old mighty magic", "mi"), "mighty");
        assert_eq!(first_rune("old magic", "zz"), "");
    }

    #[test]
    fn pick_outlives() {
        let keep = String::from("crown");
        let result;
        {
            let temp = String::from("rubble");
            result = pick(&keep, &temp);
        }
        assert_eq!(result, "crown");
    }
}
