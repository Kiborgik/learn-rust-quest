// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 4-5 · Debug Sight                                100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Your own types can't be printed, compared or copied out of the
//   box. You opt in to these abilities with `#[derive(...)]`, and the
//   compiler writes the code for you:
//
//     #[derive(Debug, Clone, PartialEq)]
//     struct Rune { name: String, power: u32 }
//
//     let r = Rune { name: String::from("Ur"), power: 3 };
//     println!("{:?}", r);    // Rune { name: "Ur", power: 3 }       (Debug)
//     println!("{:#?}", r);   // pretty, multi-line                  (Debug)
//     let r2 = r.clone();     // deep copy                           (Clone)
//     r == r2;                // field-by-field comparison           (PartialEq)
//
//   | derive     | gives you                                     |
//   |------------|-----------------------------------------------|
//   | Debug      | {:?} and {:#?} printing                       |
//   | Clone      | .clone()                                      |
//   | Copy       | implicit copies (needs Clone; Copy fields only) |
//   | PartialEq  | == and !=                                     |
//
//   `assert_eq!(a, b)` needs BOTH PartialEq (to compare) and Debug (to
//   print the values when the test fails).
//
//   (These are "traits"; lesson 8 shows how to write them yourself.)
//
// 🎯 YOUR TASK
//
//   Add the right `#[derive(...)]` lines so all the tests compile and pass.
//   Don't change anything else.

struct Item {
    name: String,
    weight: u32,
}

impl Item {
    fn new(name: &str, weight: u32) -> Self {
        Self { name: name.to_string(), weight }
    }
}

struct Coord(i32, i32);

fn main() {
    let rope = Item::new("Rope", 2);
    println!("{:?}", rope);
    println!("{:#?}", rope);
    let here = Coord(1, 2);
    let there = here;
    println!("{:?} {:?}", here, there);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Item::new("Rope", 2), Item::new("Rope", 2));
        assert_ne!(Item::new("Rope", 2), Item::new("Rope", 3));
    }

    #[test]
    fn cloning() {
        let a = Item::new("Lamp", 3);
        let mut b = a.clone();
        b.weight = 4;
        assert_eq!(a.weight, 3);
        assert_eq!(b.weight, 4);
    }

    #[test]
    fn debug_sight() {
        assert_eq!(format!("{:?}", Item::new("Rope", 2)), "Item { name: \"Rope\", weight: 2 }");
    }

    #[test]
    fn pretty_sight() {
        let s = format!("{:#?}", Item::new("Rope", 2));
        assert_eq!(s, "Item {\n    name: \"Rope\",\n    weight: 2,\n}");
    }

    #[test]
    fn copying_coords() {
        let a = Coord(1, 2);
        let b = a; // copy, not move
        assert_eq!(a, b);
        assert_eq!(format!("{:?}", b), "Coord(1, 2)");
    }
}
