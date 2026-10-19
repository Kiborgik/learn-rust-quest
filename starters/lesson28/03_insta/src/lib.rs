// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 28-3 · The Painted Record                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Some output is tedious to assert line by line: reports, rendered
//   text, big Debug dumps. SNAPSHOT tests store the expected output and
//   compare against it. `insta` keeps it right in your test (inline):
//
//     insta::assert_snapshot!(render(&board), @r"
//     [ ] #1 Slay the slime
//     [x] #2 Find the cat
//     ");
//
//     insta::assert_debug_snapshot!(hero, @"...");   // uses {:#?}
//
//   (insta strips the common indentation, so the snapshot can be indented
//   like the code around it.)
//
//   The workflow: write `@""`, run the test, READ the output it prints,
//   and if it's right, make it the snapshot. With the `cargo-insta` tool,
//   `cargo insta review` does the pasting for you. Here, copy it by hand.
//
//   When the code changes, a failing snapshot shows a diff. Then you
//   decide: bug (fix the code) or intended change (update the snapshot).
//
// 🎯 YOUR TASK
//
//   1. `board` has a snapshot already, and the code doesn't match it.
//      Read the diff and fix the code (the snapshot is right).
//   2. `receipt` has an empty snapshot. Run the test, check the output is
//      a correct receipt, and fill it in.

pub type Quest = (u32, &'static str, u32, bool);

pub fn render_board(quests: &[Quest]) -> String {
    quests
        .iter()
        .map(|(id, title, gold, done)| {
            let mark = if *done { ' ' } else { 'x' };
            format!("[{mark}] #{id} {title:<16} {gold:>4}g")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_receipt(hero: &str, items: &[(&str, u32)]) -> String {
    let mut out = format!("Receipt for {hero}\n");
    for (item, price) in items {
        out += &format!("  {item:<12}{price:>5}\n");
    }
    let total: u32 = items.iter().map(|(_, p)| p).sum();
    out += &format!("  {:<12}{total:>5}", "TOTAL");
    out
}

// ─── Tests: change only the empty snapshot! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board() {
        let quests = [(1, "Slay the slime", 50, false), (2, "Find the cat", 5, true), (3, "Guard the gate", 120, false)];
        insta::assert_snapshot!(render_board(&quests), @r"
        [ ] #1 Slay the slime     50g
        [x] #2 Find the cat        5g
        [ ] #3 Guard the gate    120g
        ");
    }

    #[test]
    fn receipt() {
        let r = render_receipt("Ferris", &[("sword", 50), ("tiny hat", 7), ("potion", 12)]);
        insta::assert_snapshot!(r, @"");
    }
}
