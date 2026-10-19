// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 8-1 · Learning a Trade                           125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A TRAIT is a set of methods that a type promises to provide, much
//   like an interface:
//
//     trait Speak {
//         fn speak(&self) -> String;          // just the signature
//     }
//
//     struct Cat;
//     struct Robot { id: u32 }
//
//     impl Speak for Cat {
//         fn speak(&self) -> String { String::from("meow") }
//     }
//     impl Speak for Robot {
//         fn speak(&self) -> String { format!("BEEP {}", self.id) }
//     }
//
//     Cat.speak();                // "meow"
//
//   • `impl Trait for Type` lives OUTSIDE the type definition. You can
//     even add your own traits to std types: `impl Speak for i32 { .. }`
//   • Every method of the trait must be implemented (unless it has a
//     default, which is the next quest).
//   • The trait must be in scope (`use`) to call its methods. Here
//     everything is in one file, so it always is.
//
// 🎯 YOUR TASK
//
//   1. Implement `Craft` for `Blacksmith` and `Alchemist`.
//      Blacksmith: product "Sword", cost = 10 * skill
//      Alchemist:  product "Potion", cost = 5 + potions_brewed
//   2. Implement `Craft` for `u32` too (yes, a built-in type!):
//      product "Coin", cost = the number itself.

trait Craft {
    fn product(&self) -> String;
    fn cost(&self) -> u32;
}

struct Blacksmith {
    skill: u32,
}

struct Alchemist {
    potions_brewed: u32,
}

impl Craft for Blacksmith {
    fn product(&self) -> String {
        String::from("Sword")
    }
}

fn main() {
    let b = Blacksmith { skill: 3 };
    println!("{} costs {}", b.product(), b.cost());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blacksmith() {
        let b = Blacksmith { skill: 3 };
        assert_eq!(b.product(), "Sword");
        assert_eq!(b.cost(), 30);
    }

    #[test]
    fn alchemist() {
        let a = Alchemist { potions_brewed: 7 };
        assert_eq!(a.product(), "Potion");
        assert_eq!(a.cost(), 12);
    }

    #[test]
    fn coins() {
        assert_eq!(5u32.product(), "Coin");
        assert_eq!(5u32.cost(), 5);
    }
}
