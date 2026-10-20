// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 8-4 · Bound by Oath                              125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Several ways to write the same bound:
//
//     fn show<T: Display>(x: T) -> String { format!("[{x}]") }
//
//     fn show(x: impl Display) -> String { format!("[{x}]") }   // shorthand
//
//     fn show<T>(x: T) -> String
//     where
//         T: Display,                    // `where`: tidy for many bounds
//     { format!("[{x}]") }
//
//   Multiple bounds use `+`:
//
//     fn log<T: Display + Clone>(x: &T) { let copy = x.clone(); }
//
//   Bounds work with YOUR traits too:
//
//     trait Named { fn name(&self) -> String; }
//     fn hello(x: &impl Named) -> String { format!("hi {}", x.name()) }
//
//   Useful std traits for bounds: Display ({}), Debug ({:?}), Clone,
//   Copy, PartialEq (==), PartialOrd (<, >), Default, std::ops::Add (+)...
//
// 🎯 YOUR TASK
//
//   Add the missing bounds. Read each compiler error: it usually
//   suggests the exact bound to add.
//   Then implement `loudest`, which returns the name of the creature
//   with the highest volume.

use std::fmt::Debug;
use std::fmt::Display;

trait Creature {
    fn name(&self) -> String;
    fn volume(&self) -> u32;
}

struct Wolf;
struct Mouse;

impl Creature for Wolf {
    fn name(&self) -> String { String::from("Wolf") }
    fn volume(&self) -> u32 { 90 }
}
impl Creature for Mouse {
    fn name(&self) -> String { String::from("Mouse") }
    fn volume(&self) -> u32 { 5 }
}

/// "<<value>>"
fn banner<T>(value: T) -> String {
    format!("<<{value}>>")
}

/// Returns two copies of the value, in a tuple.
fn twins<T>(value: T) -> (T, T) {
    (value.clone(), value)
}

/// "Wolf: 90"
fn report(c: &impl Creature) -> String {
    format!("{}: {}", c.name(), c.volume())
}

/// Debug-prints both values, and says whether they are equal.
fn compare<A>(a: A, b: A) -> String
where
    A: Debug,
{
    format!("{:?} vs {:?}: {}", a, b, a == b)
}

/// The name of the louder of two creatures (the first one on a tie).
fn loudest(a: &impl Creature, b: &impl Creature) -> String {
    todo!()
}

fn main() {
    println!("{}", banner(7));
    println!("{}", report(&Wolf));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banners() {
        assert_eq!(banner(7), "<<7>>");
        assert_eq!(banner("hi"), "<<hi>>");
    }

    #[test]
    fn twin_values() {
        assert_eq!(twins(String::from("x")), (String::from("x"), String::from("x")));
    }

    #[test]
    fn reports() {
        assert_eq!(report(&Wolf), "Wolf: 90");
    }

    #[test]
    fn comparing() {
        assert_eq!(compare(1, 1), "1 vs 1: true");
        assert_eq!(compare("a", "b"), "\"a\" vs \"b\": false");
    }

    #[test]
    fn loudness() {
        assert_eq!(loudest(&Mouse, &Wolf), "Wolf");
        assert_eq!(loudest(&Wolf, &Mouse), "Wolf");
        assert_eq!(loudest(&Mouse, &Mouse), "Mouse");
    }
}
