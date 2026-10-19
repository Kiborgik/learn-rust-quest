// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Grimoire                                  400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The guild scribes want to write quest logs in their own little
//   language, checked by the compiler:
//
//     let log = quest_log! {
//         "Slay the slime" => 50 gold,
//         "Find the cat"   => 5 gold [done],
//     };
//
//   ⚔️  HEAD 1: `quest_log!` turns each line into an `Entry` and returns a
//        Vec<Entry>. `title => N gold` (the word `gold` is required).
//
//   ⚔️  HEAD 2: an optional `[done]` after the gold marks it done. Tip:
//        capture `$([$flag:ident])?` and pass it to a helper rule:
//          (@done) => { false };
//          (@done done) => { true };
//
//   ⚔️  HEAD 3: trailing comma allowed, and `quest_log! {}` is empty.
//
//   ⚔️  HEAD 4: `ensure!(condition, "format {}", args...)`: if the
//        condition is false, `return Err(format!(...))` from the function
//        it's used in. Capture the format part with `$($msg:tt)+` and
//        pass it on to format!.
//
//   Replace the placeholder macros below.

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub title: &'static str,
    pub gold: u32,
    pub done: bool,
}

macro_rules! quest_log {
    ($($anything:tt)*) => {
        Vec::<Entry>::new()
    };
}

macro_rules! ensure {
    ($($anything:tt)*) => {};
}

fn main() {
    let log = quest_log! {
        "Slay the slime" => 50 gold,
        "Find the cat" => 5 gold [done],
    };
    println!("{log:?}");
}

// ─── The Grimoire's tests: don't change these! ───────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn entry(title: &'static str, gold: u32, done: bool) -> Entry {
        Entry { title, gold, done }
    }

    #[test]
    fn head_1_entries() {
        let log = quest_log! { "Slay the slime" => 50 gold };
        assert_eq!(log, [entry("Slay the slime", 50, false)]);
    }

    #[test]
    fn head_2_done() {
        let log = quest_log! {
            "Slay the slime" => 50 gold,
            "Find the cat" => 5 gold [done],
            "Guard the gate" => 20 gold
        };
        assert_eq!(
            log,
            [entry("Slay the slime", 50, false), entry("Find the cat", 5, true), entry("Guard the gate", 20, false)]
        );
    }

    #[test]
    fn head_3_commas_and_empty() {
        let log = quest_log! { "a" => 1 gold [done], };
        assert_eq!(log.len(), 1);
        let empty: Vec<Entry> = quest_log! {};
        assert!(empty.is_empty());
    }

    fn pay(log: &[Entry], budget: u32) -> Result<u32, String> {
        let total: u32 = log.iter().filter(|e| e.done).map(|e| e.gold).sum();
        ensure!(total > 0, "nothing to pay");
        ensure!(total <= budget, "need {} gold but have {}", total, budget);
        Ok(total)
    }

    #[test]
    fn head_4_ensure() {
        let log = quest_log! { "a" => 30 gold [done], "b" => 50 gold [done], "c" => 99 gold };
        assert_eq!(pay(&log, 100), Ok(80));
        assert_eq!(pay(&log, 10), Err("need 80 gold but have 10".to_string()));
        assert_eq!(pay(&[], 10), Err("nothing to pay".to_string()));
    }
}
