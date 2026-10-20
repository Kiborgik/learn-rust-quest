// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 8-2 · Inherited Wisdom                           125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Traits can provide DEFAULT method bodies. Implementors get them for
//   free and can override them:
//
//     trait Greet {
//         fn name(&self) -> String;                   // required
//
//         fn greet(&self) -> String {                 // default
//             format!("Hello, {}!", self.name())      // may call required ones
//         }
//     }
//
//     struct Knight;
//     impl Greet for Knight {
//         fn name(&self) -> String { String::from("Sir Crab") }
//         // greet() comes from the default
//     }
//
//     struct Pirate;
//     impl Greet for Pirate {
//         fn name(&self) -> String { String::from("Redclaw") }
//         fn greet(&self) -> String {               // override
//             String::from("Arr!")
//         }
//     }
//
//   This is how Rust shares behaviour without inheritance: a default
//   method can build on a few required methods. (`Iterator` gives you
//   70+ methods if you write just one: `next`. That's lesson 9!)
//
// 🎯 YOUR TASK
//
//   1. Give `Mentor` a DEFAULT `teach` method returning
//      "{name} teaches {subject}" built from the two required methods.
//   2. Give `Mentor` a DEFAULT `wisdom` method returning `years() * 10`.
//   3. `Sage` overrides `wisdom`: it is 1000, no matter what.
//   Note: `Elder` and `Sage` below must NOT define `teach`.

trait Mentor {
    fn name(&self) -> String;
    fn subject(&self) -> String;
    fn years(&self) -> u32;
    // TODO: default `teach` and `wisdom`
}

struct Elder {
    years: u32,
}

struct Sage;

impl Mentor for Elder {
    fn name(&self) -> String {
        String::from("Elder Oak")
    }
    fn subject(&self) -> String {
        String::from("patience")
    }
    fn years(&self) -> u32 {
        self.years
    }
}

impl Mentor for Sage {
    fn name(&self) -> String {
        String::from("The Sage")
    }
    fn subject(&self) -> String {
        String::from("everything")
    }
    fn years(&self) -> u32 {
        3
    }
}

fn main() {
    let e = Elder { years: 70 };
    println!("{} (wisdom {})", e.teach(), e.wisdom());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elder_uses_defaults() {
        let e = Elder { years: 70 };
        assert_eq!(e.teach(), "Elder Oak teaches patience");
        assert_eq!(e.wisdom(), 700);
    }

    #[test]
    fn sage_overrides() {
        assert_eq!(Sage.teach(), "The Sage teaches everything");
        assert_eq!(Sage.wisdom(), 1000);
    }
}
