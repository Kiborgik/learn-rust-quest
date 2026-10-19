// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 1-5 · The Type Smith                             100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Scalar types (a single value):
//     let a: i32  = -42;       // signed integers:   i8 i16 i32 i64 i128 isize
//     let b: u8   = 255;       // unsigned (≥ 0):    u8 u16 u32 u64 u128 usize
//     let c: f64  = 3.14;      // floating point:    f32 f64
//     let d: bool = true;      // true / false
//     let e: char = '🦀';      // ONE Unicode character, in SINGLE quotes
//     let f: &str = "hello";   // text (a string slice), in DOUBLE quotes
//
//   Compound types (several values):
//     let pos: (i32, i32) = (3, -4);       // tuple: mixed types allowed
//     println!("{}", pos.0);               // → 3   (fields are .0, .1, ...)
//
//     let loot: [u32; 3] = [10, 20, 30];   // array: same type, fixed length
//     println!("{}", loot[0]);             // → 10  (indexes start at 0!)
//
//   Integer ranges:  u8 = 0..=255,  i8 = -128..=127,  u16 = 0..=65535 ...
//
// 📖 NUMBERS: THE THREE GOTCHAS
//
//   1. NO IMPLICIT CONVERSIONS, not even "safe" widening:
//        let small: u32 = 7;
//        let big: u64 = small;           // ✘ ERROR: expected u64, found u32
//        let big: u64 = small as u64;    // ✔ explicit cast
//        let big = u64::from(small);     // ✔ also fine (lossless only)
//        let x = 7 / 2;                  // 3: integer division truncates
//        let y = 7 as f64 / 2.0;         // 3.5
//
//   2. OVERFLOW IS AN ERROR, not silent wrap-around:
//        let hp: u8 = 250;
//        hp + 10            // ✘ compile error if the compiler can see it,
//                           //   otherwise a PANIC at runtime (debug builds)
//        hp.saturating_add(10)   // 255  (stops at the max)
//        hp.wrapping_add(10)     // 4    (wraps around, like C)
//
//   3. `as` NEVER FAILS, it can silently lose data:
//        300_i32 as u8      // 44   (keeps the low 8 bits)
//        -1_i32 as u32      // 4294967295
//        3.99_f64 as i32    // 3    (truncates toward zero)
//
// 🎯 YOUR TASK
//
//   The blacksmith mixed up the molds. Fix the TYPES, the two bugs in
//   the `println!`s, and the treasury math, so the program prints:
//
//       HP: 300, speed: 1.5, alive: true, rune: R
//       Position: x=10 y=-3
//       Last item: potion
//       Treasury: 10000001234
//       Rage: 255

fn main() {
    let max_hp: u8 = 300;
    let speed: i32 = 1.5;
    let is_alive: bool = "true";
    let rune: char = "R";
    let position: (i32, i32) = (10, -3);
    let inventory: [&str; 3] = ["sword", "shield", "potion"];

    println!("HP: {max_hp}, speed: {speed}, alive: {is_alive}, rune: {rune}");
    println!("Position: x={} y={}", position.0, position.0);
    println!("Last item: {}", inventory[3]);

    let pocket_gold: u32 = 1234;
    let royal_vault: u64 = 10_000_000_000; // `_` in numbers is just for readability
    let treasury: u64 = pocket_gold + royal_vault;
    println!("Treasury: {treasury}");

    let rage: u8 = 250;
    let berserk = rage + 10; // rage can't go above 255. Cap it, don't crash!
    println!("Rage: {berserk}");
}
