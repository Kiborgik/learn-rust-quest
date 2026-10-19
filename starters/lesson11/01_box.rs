// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 11-1 · The Box of Holding                        150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `Box<T>` puts a value on the HEAP and owns it. The box itself is
//   just a pointer, with a fixed size:
//
//     let b = Box::new(5);          // an i32 on the heap
//     let x = *b + 1;               // deref with `*`, like a reference
//
//   Most of the time you don't need it. You DO need it for RECURSIVE
//   types, whose size would otherwise be infinite:
//
//     enum List {
//         Cons(i32, List),          // ✘ ERROR E0072: recursive type has infinite size
//         Nil,
//     }
//
//     enum List {
//         Cons(i32, Box<List>),     // ✔ a pointer has a known size
//         Nil,
//     }
//     let l = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
//
//   Matching through a Box: `match list { List::Cons(v, rest) => ... }`
//   gives `rest: &Box<List>`, and you can call functions on it directly
//   (`sum(rest)`): Rust auto-derefs `&Box<List>` to `&List`.
//
// 🎯 YOUR TASK
//
//   An `Expr` is a tiny arithmetic expression tree: (2 + 3) * 4
//   1. Make the type compile (box the recursive parts).
//   2. Implement `eval` recursively.
//   3. Implement `count_numbers`: how many `Num` leaves the tree has.

#[derive(Debug)]
enum Expr {
    Num(i64),
    Add(Expr, Expr),
    Mul(Expr, Expr),
    Neg(Expr),
}

use Expr::*;

fn eval(e: &Expr) -> i64 {
    todo!()
}

fn count_numbers(e: &Expr) -> usize {
    todo!()
}

/// (2 + 3) * 4
fn example() -> Expr {
    Mul(Box::new(Add(Box::new(Num(2)), Box::new(Num(3)))), Box::new(Num(4)))
}

fn main() {
    println!("{:?} = {}", example(), eval(&example()));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluating() {
        assert_eq!(eval(&example()), 20);
        assert_eq!(eval(&Neg(Box::new(Num(7)))), -7);
        assert_eq!(eval(&Num(42)), 42);
    }

    #[test]
    fn counting() {
        assert_eq!(count_numbers(&example()), 3);
        assert_eq!(count_numbers(&Neg(Box::new(Num(1)))), 1);
    }
}
