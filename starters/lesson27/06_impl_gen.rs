// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 27-6 · The Copying Spell                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The standard library implements traits like Add or Display for every
//   number type. Nobody wrote twelve copies by hand: a macro did.
//
//     trait Half { fn half(self) -> Self; }
//
//     macro_rules! impl_half {
//         ($($t:ty),* $(,)?) => {
//             $( impl Half for $t { fn half(self) -> Self { self / 2 as $t } } )*
//         };
//     }
//     impl_half!(u8, u16, u32, f64);
//
//   The repetition wraps the WHOLE impl block, so one call makes one impl
//   per type.
//
//   You can repeat pairs too: `$($t:ty => $label:literal),*`.
//
//   (Before reaching for a macro, check if a generic impl works:
//   `impl<T: Display> Describe for T`. Use a macro when the bodies differ
//   per type in a way generics can't express, like `0 as $t`.)
//
// 🎯 YOUR TASK
//
//   1. `impl_zero!` handles one type. Make it take a list.
//   2. Write `impl_kind!` for `Kind`, taking pairs `type => "label"`, and
//      implement it for i32, u8, f64 and String.

trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

trait Kind {
    fn kind() -> &'static str;
}

macro_rules! impl_zero {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0 as $t
            }
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    };
}

impl_zero!(i32);

fn main() {
    println!("{}", i32::zero());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros() {
        assert_eq!(i32::zero(), 0);
        assert_eq!(i64::zero(), 0);
        assert_eq!(u8::zero(), 0);
        assert_eq!(f64::zero(), 0.0);
        assert!(0.0f64.is_zero());
        assert!(!7u8.is_zero());
    }

    #[test]
    fn kinds() {
        assert_eq!(i32::kind(), "integer");
        assert_eq!(u8::kind(), "byte");
        assert_eq!(f64::kind(), "float");
        assert_eq!(String::kind(), "text");
    }
}
