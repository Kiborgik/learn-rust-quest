// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 4-2 · Coins of the Realm                         100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   TUPLE STRUCTS have fields without names, accessed with .0, .1, ...
//
//     struct Rgb(u8, u8, u8);
//     let red = Rgb(255, 0, 0);
//     println!("{}", red.0);          // 255
//
//   The NEWTYPE pattern wraps one value in its own type, so the compiler
//   stops you from mixing things up that happen to share a type:
//
//     struct Meters(f64);
//     struct Feet(f64);
//     fn climb(height: Meters) { }
//     climb(Feet(30.0));              // ✘ ERROR: expected Meters, found Feet
//
//   It costs nothing at runtime: a `Meters` is just an f64 in memory.
//
//   UNIT STRUCTS have no fields at all:  `struct Marker;`
//   (useful later with traits).
//
//   `.abs()` gives the absolute value of a signed number: (-3i32).abs() == 3
//
// 🎯 YOUR TASK
//
//   1. Fix `gold_to_silver` (you can't multiply a `Gold` directly).
//   2. Fix `treasure`: the arguments are in the wrong order, and the
//      newtypes caught the bug!
//   3. Implement `manhattan`: |x| + |y| + |z| of a `Point3`.

struct Gold(u32);
struct Silver(u32);

/// 1 gold = 100 silver
fn gold_to_silver(g: Gold) -> Silver {
    Silver(g * 100)
}

/// Total value of a purse, counted in silver.
fn total_value(gold: Gold, silver: Silver) -> u32 {
    gold.0 * 100 + silver.0
}

fn treasure() -> u32 {
    total_value(Silver(50), Gold(2))
}

struct Point3(i32, i32, i32);

fn manhattan(p: &Point3) -> i32 {
    todo!()
}

fn main() {
    println!("3 gold = {} silver", gold_to_silver(Gold(3)).0);
    println!("Treasure: {} silver", treasure());
    println!("Distance: {}", manhattan(&Point3(1, -2, 3)));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exchange() {
        assert_eq!(gold_to_silver(Gold(3)).0, 300);
    }

    #[test]
    fn treasure_value() {
        assert_eq!(treasure(), 250);
    }

    #[test]
    fn distances() {
        assert_eq!(manhattan(&Point3(1, -2, 3)), 6);
        assert_eq!(manhattan(&Point3(0, 0, 0)), 0);
        assert_eq!(manhattan(&Point3(-5, -5, -5)), 15);
    }
}
