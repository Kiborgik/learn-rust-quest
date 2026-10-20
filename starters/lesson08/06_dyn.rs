// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 8-6 · The Masquerade                             125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Generics are resolved at COMPILE time, so a `Vec<T>` holds ONE
//   type. To mix different types that share a trait, use a TRAIT
//   OBJECT `dyn Trait`, which is resolved at RUNTIME through a vtable,
//   like virtual methods in C++/Java/C#:
//
//     trait Dancer { fn dance(&self) -> String; }
//
//     let party: Vec<Box<dyn Dancer>> = vec![
//         Box::new(Crab),                // Box puts it on the heap
//         Box::new(Robot { id: 7 }),     // different type, same trait ✔
//     ];
//     for d in &party { d.dance(); }     // dynamic dispatch
//
//     fn spotlight(d: &dyn Dancer) { }   // borrow any dancer
//
//   Why `Box`? Different types have different sizes. A Vec needs
//   same-sized elements, and a Box (a pointer) always has the same size.
//
//   | static: `impl Trait` / `<T: Trait>` | dynamic: `dyn Trait`        |
//   |--------------------------------------|----------------------------|
//   | one concrete type per call site      | mix types at runtime        |
//   | fastest, can be inlined              | small vtable cost           |
//
//   Returning "some type implementing a trait":
//     fn make() -> impl Dancer { Crab }          // ONE hidden concrete type
//     fn pick(b: bool) -> Box<dyn Dancer> {      // may differ per call ✔
//         if b { Box::new(Crab) } else { Box::new(Robot { id: 1 }) }
//     }
//
// 🎯 YOUR TASK
//
//   1. Fix `guests`: it must hold different mask types.
//   2. Implement `total_charm(guests: &[Box<dyn Mask>]) -> u32`.
//   3. Fix `random_mask` so it can return either type.

trait Mask {
    fn describe(&self) -> String;
    fn charm(&self) -> u32;
}

struct Fox;
struct Owl {
    wisdom: u32,
}

impl Mask for Fox {
    fn describe(&self) -> String { String::from("a sly fox") }
    fn charm(&self) -> u32 { 7 }
}
impl Mask for Owl {
    fn describe(&self) -> String { format!("an owl ({} wisdom)", self.wisdom) }
    fn charm(&self) -> u32 { self.wisdom / 2 }
}

fn guests() -> Vec<Box<Fox>> {
    vec![Box::new(Fox), Box::new(Owl { wisdom: 10 }), Box::new(Fox)]
}

fn total_charm(guests: &[Box<dyn Mask>]) -> u32 {
    todo!()
}

fn random_mask(roll: u32) -> impl Mask {
    if roll % 2 == 0 {
        Fox
    } else {
        Owl { wisdom: roll }
    }
}

fn main() {
    for g in &guests() {
        println!("Arriving: {}", g.describe());
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_party() {
        let g = guests();
        assert_eq!(g.len(), 3);
        assert_eq!(g[1].describe(), "an owl (10 wisdom)");
        assert_eq!(total_charm(&g), 19);
    }

    #[test]
    fn masks() {
        assert_eq!(random_mask(2).describe(), "a sly fox");
        assert_eq!(random_mask(9).describe(), "an owl (9 wisdom)");
    }
}
