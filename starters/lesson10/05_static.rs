// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 10-5 · Eternal Scrolls                           150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `'static` is a special lifetime: "valid for the whole program".
//
//     let s: &'static str = "hello";    // literals are baked into the binary
//
//     const MAX_LEVEL: u32 = 99;              // inlined everywhere it's used
//     static GREETING: &str = "Welcome!";     // one fixed memory location
//
//   Two different meanings to keep apart:
//
//   • `&'static T`: a reference that's valid forever (literals, statics).
//   • `T: 'static` as a BOUND: "T contains no short-lived references".
//     Every OWNED type satisfies it: String, Vec<u32>, i32... A String
//     doesn't live forever, but it doesn't BORROW anything either.
//     `thread::spawn` requires `'static` (lesson 12!).
//
//   ⚠️ Myth: "add 'static to make the error go away". It usually just
//   moves the error somewhere else. If you need data to live longer,
//   OWN it instead.
//
// 🎯 YOUR TASK
//
//   1. `rank_name` returns literal text. Give it the right return type.
//   2. `RANKS` must be a `static` array of 3 names:
//      "Squire", "Knight", "Paladin"   (used by `rank_name`)
//   3. `keep_forever` stores things in a `Vault`, which requires
//      `T: 'static`. The test passes a String and fails. Why? Look at
//      `remember`: it takes a REFERENCE. Make it take an owned value instead.

fn rank_name(level: usize) -> &str {
    if level < RANKS.len() { RANKS[level] } else { "Legend" }
}

struct Vault<T: 'static> {
    items: Vec<T>,
}

impl<T: 'static> Vault<T> {
    fn new() -> Self {
        Vault { items: Vec::new() }
    }
    fn store(&mut self, item: T) {
        self.items.push(item);
    }
}

fn remember(vault: &mut Vault<&String>, memory: &String) {
    vault.store(memory);
}

fn main() {
    println!("{}", rank_name(1));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks() {
        assert_eq!(rank_name(0), "Squire");
        assert_eq!(rank_name(2), "Paladin");
        assert_eq!(rank_name(9), "Legend");
        assert_eq!(RANKS.len(), 3);
    }

    #[test]
    fn vault() {
        let mut vault = Vault::new();
        let memory = String::from("first quest");
        remember(&mut vault, memory);
        remember(&mut vault, String::from("first boss"));
        assert_eq!(vault.items, vec!["first quest", "first boss"]);
    }
}
