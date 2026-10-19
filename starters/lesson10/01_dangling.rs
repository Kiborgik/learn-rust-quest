// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 10-1 · Ghost References                          150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A dangling reference points at memory that has already been freed.
//   In C it's a classic source of crashes and security holes. Rust
//   refuses to compile one:
//
//     fn ghost() -> &String {             // ✘ ERROR: missing lifetime specifier
//         let s = String::from("boo");
//         &s                              // s is dropped at the end of this
//     }                                   //   function, so the ref would dangle
//
//     fn real() -> String {               // ✔ return the OWNED value instead
//         String::from("boo")
//     }
//
//   Same thing inside one function:
//
//     let r;
//     {
//         let x = 5;
//         r = &x;                // ✘ ERROR: `x` does not live long enough
//     }                          // x dropped here...
//     println!("{r}");           // ...but r is still used here
//
//   Every reference has a LIFETIME: the region of code where it's valid.
//   The compiler checks that no reference is used outside the lifetime
//   of the value it points to. Usually you never write lifetimes down;
//   this lesson is about the cases where you do.
//
// 🎯 YOUR TASK
//
//   1. `summon_name` returns a reference to a local. Return an owned
//      `String` instead.
//   2. `longest_scroll`: `best` points at a value that dies too early.
//      Restructure the code so it compiles: declare the scrolls in the
//      outer scope. Don't use .clone().

fn summon_name(level: u32) -> &str {
    let name = format!("Ghost of level {level}");
    &name
}

fn longest_scroll() -> usize {
    let short = String::from("boo");
    let best: &String;
    {
        let long = String::from("BOOOOOOO");
        best = if long.len() > short.len() { &long } else { &short };
    }
    best.len()
}

fn main() {
    println!("{}", summon_name(3));
    println!("{}", longest_scroll());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(summon_name(3), "Ghost of level 3");
    }

    #[test]
    fn scrolls() {
        assert_eq!(longest_scroll(), 8);
    }
}
