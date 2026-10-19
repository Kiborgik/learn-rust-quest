// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 27-3 · Shapes of Words                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   The part after the colon is a FRAGMENT SPECIFIER. It says what kind
//   of code the macro accepts:
//
//     $e:expr      an expression         2 + 3, foo(), "hi"
//     $i:ident     a name                Hero, level, my_fn
//     $t:ty        a type                u32, Vec<String>, &str
//     $l:literal   a literal             42, "text", true
//     $b:block     a { ... } block
//     $p:pat       a pattern             Some(x), 1..=5
//     $tt:tt       one token tree        anything (the wildcard)
//
//   With `ident` and `ty`, a macro can write whole items: structs, fns,
//   impls. That's how a lot of boilerplate disappears:
//
//     macro_rules! getter {
//         ($field:ident: $ty:ty) => {
//             fn $field(&self) -> &$ty { &self.$field }
//         };
//     }
//
//   `stringify!(...)` turns tokens into a &'static str at compile time:
//   stringify!(level) == "level".
//
// 🎯 YOUR TASK
//
//   1. `record!` uses the wrong fragment kinds, so the struct can't be
//      built. A struct name and field names are idents; field types are
//      types.
//   2. Add `fn field_names() -> &'static [&'static str]` to the generated
//      impl, listing the fields in order.

macro_rules! record {
    ($name:expr { $($field:expr : $ty:expr),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $( pub $field: $ty ),*
        }

        impl $name {
            pub fn new($( $field: $ty ),*) -> Self {
                Self { $( $field ),* }
            }
        }
    };
}

record!(Hero { name: String, level: u32 });
record!(Item { title: &'static str, price: u32, magic: bool });

fn main() {
    let h = Hero::new("Ferris".into(), 12);
    println!("{h:?}");
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heroes() {
        let h = Hero::new("Ferris".into(), 12);
        assert_eq!(h.name, "Ferris");
        assert_eq!(h.level, 12);
        assert_eq!(Hero::field_names(), ["name", "level"]);
    }

    #[test]
    fn items() {
        let i = Item::new("wand", 90, true);
        assert!(i.magic);
        assert_eq!(Item::field_names(), ["title", "price", "magic"]);
    }
}
