// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 19-1 · The Bag of Holding                        175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   An ASSOCIATED TYPE is a type slot inside a trait, filled in by each
//   implementation. You've used one already: `Iterator`'s `type Item`.
//
//     trait Container {
//         type Item;                                  // "some type, decided per impl"
//         fn get(&self, i: usize) -> Option<&Self::Item>;
//         fn first(&self) -> Option<&Self::Item> {    // defaults can use it
//             self.get(0)
//         }
//     }
//
//     impl Container for Quiver {
//         type Item = Arrow;                          // filled in here
//         fn get(&self, i: usize) -> Option<&Arrow> { self.arrows.get(i) }
//     }
//
//   Associated type vs generic trait:
//
//     trait Container { type Item; }     // ONE impl per type: a Quiver holds Arrows
//     trait From<T> { ... }              // MANY impls per type: From<u32>, From<&str>...
//
//   Use an associated type when there's only one sensible answer. Callers
//   then don't have to spell it out: `fn f(c: &impl Container)`.
//
//   Bounds on it: `fn f<C: Container<Item = String>>(c: &C)`.
//
// 🎯 YOUR TASK
//
//   1. `Backpack` doesn't say what it holds. Complete its impl.
//   2. Implement `Container` for `Wallet` (it holds u32 coins).
//   3. Write the default method `count`: how many items the container has
//      (call `get` with 0, 1, 2, ... until it returns None).

trait Container {
    type Item;
    fn get(&self, i: usize) -> Option<&Self::Item>;

    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }

    // TODO: fn count(&self) -> usize with a default body
}

struct Backpack {
    items: Vec<String>,
}

struct Wallet {
    coins: [u32; 3],
}

impl Container for Backpack {
    fn get(&self, i: usize) -> Option<&String> {
        self.items.get(i)
    }
}

fn describe_first<C: Container<Item = String>>(c: &C) -> String {
    match c.first() {
        Some(s) => format!("first: {s}"),
        None => "empty".to_string(),
    }
}

fn main() {
    let bag = Backpack { items: vec!["rope".into(), "torch".into()] };
    println!("{}", describe_first(&bag));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backpack() {
        let bag = Backpack { items: vec!["rope".into(), "torch".into()] };
        assert_eq!(describe_first(&bag), "first: rope");
        assert_eq!(bag.count(), 2);
        assert_eq!(describe_first(&Backpack { items: vec![] }), "empty");
    }

    #[test]
    fn wallet() {
        let w = Wallet { coins: [5, 10, 1] };
        assert_eq!(w.first(), Some(&5));
        assert_eq!(w.get(2), Some(&1));
        assert_eq!(w.count(), 3);
    }
}
