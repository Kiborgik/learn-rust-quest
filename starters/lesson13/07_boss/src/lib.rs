// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Dice Tower                                350 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Dice Tower decides every fate in the realm. Build it as a real
//   library crate with three modules. The signatures are written; the
//   bodies are `todo!()`. The tests in `tests/tower.rs` are the spec.
//
//   ⚔️  HEAD 1: wiring. The tests can't find the `tower` module. Fix this
//        file.
//
//   ⚔️  HEAD 2: `src/dice.rs`. `Die::new(sides)` gives `None` for fewer than
//        2 sides. `roll` returns 1..=sides.
//
//   ⚔️  HEAD 3: `src/notation.rs`. `parse` reads dice notation:
//          "2d6+3"  → 2 dice, 6 sides, bonus +3
//          "d20"    → 1 die (count is optional), 20 sides
//          "1d4-1"  → bonus -1
//        Errors, checked in this order:
//          blank text              → ParseError::Empty
//          no 'd'                  → ParseError::MissingD
//          count isn't a number ≥1 → ParseError::BadNumber(that text)
//          sides or bonus not a number → ParseError::BadNumber(that text)
//          fewer than 2 sides      → ParseError::BadDie(sides)
//        Plus `min`, `max`, `roll`, and `Display` ("2d6+3", "1d20", "1d4-1").
//
//   ⚔️  HEAD 4: `src/tower.rs`. `Tower::parse("2d6+3, d20")` (comma
//        separated, spaces allowed), `max_total`, `roll_all`.
//
//   Tips: `text.split_once('d')`, `str::parse::<u32>()`, and for the bonus
//   look for '+' or '-' with `find`. `map_err` turns a parse error into
//   your own error.

pub mod dice;
pub mod notation;
