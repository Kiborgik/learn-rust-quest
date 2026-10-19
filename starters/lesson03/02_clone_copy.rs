// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 3-2 · The Mirror Forge                           100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Small, fixed-size values that live on the stack are `Copy`.
//   Assigning them COPIES instead of moving:
//
//     let hp = 100;
//     let backup = hp;        // copy, both usable ✔
//
//   Copy types: all integers, floats, bool, char, and tuples/arrays
//   made only of Copy types, like (i32, bool) or [u8; 4].
//
//   Heap-owning types like `String` are NOT Copy. If you really want a
//   second, independent copy, ask for it explicitly with `.clone()`:
//
//     let a = String::from("Shield");
//     let b = a.clone();      // deep copy: a new heap allocation
//     println!("{a} {b}");    // ✔ both usable, completely independent
//
//   Why explicit? Cloning can be expensive. Rust makes costs visible.
//
//   {:?} is the "debug" formatter, which can print tuples and other
//   compound values that {} can't.
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       hp=100, backup=100
//       pos=(3, 4), saved=(3, 4)
//       spell=Fireball, scroll=Fireball
//       For the crab!
//       For the crab!
//       original=Shield +1, copy=Shield

fn shout(s: String) {
    println!("{s}!");
}

fn main() {
    let hp = 100;
    let backup = hp;
    println!("hp={hp}, backup={backup}");

    let pos = (3, 4);
    let saved = pos;
    println!("pos={:?}, saved={:?}", pos, saved);

    let spell = String::from("Fireball");
    let scroll = spell;
    println!("spell={spell}, scroll={scroll}");

    let war_cry = String::from("For the crab");
    shout(war_cry);
    shout(war_cry);

    let mut original = String::from("Shield");
    let copy = original.clone();
    original.push_str(" +1");
    println!("original={original}, copy={copy}");
}
