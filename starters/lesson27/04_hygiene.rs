// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 27-4 · Clean Hands                               175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   HYGIENE: names a macro makes up stay inside the macro. This is safe
//   even if the caller has its own `tmp`:
//
//     macro_rules! swap {
//         ($a:ident, $b:ident) => { let tmp = $a; $a = $b; $b = tmp; };
//     }
//
//   The flip side: a macro can't create a variable for the caller with a
//   name it invents. If the caller should see the name, the CALLER passes
//   it in, as an ident:
//
//     macro_rules! make { ($name:ident = $v:expr) => { let $name = $v; }; }
//     make!(gold = 5);    // now `gold` exists here
//
//   SCOPE: macro_rules! macros aren't items with paths by default. To use
//   one from another module, re-export it after the definition:
//
//     mod spells {
//         macro_rules! bolt { ... }
//         pub(crate) use bolt;      // now `spells::bolt!(...)` works
//     }
//
//   (For other crates: `#[macro_export]`, and `$crate::` inside the macro
//   for paths that must work from anywhere.)
//
// 🎯 YOUR TASK
//
//   1. `bind!` is meant to create the variable the caller names, but it
//      always makes its own `power`, which nobody can see.
//   2. `spells::bolt!` can't be used outside the `spells` module.

macro_rules! swap {
    ($a:ident, $b:ident) => {
        let tmp = $a;
        $a = $b;
        $b = tmp;
    };
}

macro_rules! bind {
    ($name:ident = $value:expr) => {
        let power = $value;
    };
}

mod spells {
    macro_rules! bolt {
        ($target:expr) => {
            format!("⚡ {} takes {} damage", $target, 10)
        };
    }
}

fn main() {
    let (mut a, mut b) = (1, 2);
    swap!(a, b);
    println!("{a} {b}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swapping_is_clean() {
        let tmp = "mine";
        let (mut a, mut b) = (1, 2);
        swap!(a, b);
        assert_eq!((a, b, tmp), (2, 1, "mine"));
    }

    #[test]
    fn binding() {
        bind!(power = 5);
        bind!(mana = power * 2);
        assert_eq!(power + mana, 15);
    }

    #[test]
    fn bolts() {
        assert_eq!(spells::bolt!("the slime"), "⚡ the slime takes 10 damage");
    }
}
