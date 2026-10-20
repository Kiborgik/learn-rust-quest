// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 6-4 · The Price Ledger                           125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//     use std::collections::HashMap;          // not in the prelude
//
//     let mut prices: HashMap<String, u32> = HashMap::new();
//     prices.insert(String::from("apple"), 3);   // insert / overwrite
//
//     prices.get("apple");          // Option<&u32>: Some(&3)
//     prices.get("pear");           // None
//     prices["apple"];             // 3, PANICS if it's missing
//     prices.contains_key("apple"); // true
//     prices.remove("apple");       // Option<u32>: the removed value
//     prices.len();
//
//     if let Some(p) = prices.get("apple") {  // p: &u32
//         println!("{p}");
//     }
//     let p = prices.get("apple").copied().unwrap_or(0); // u32
//
//     for (name, price) in &prices { }   // ⚠️ the ORDER IS RANDOM
//
//   • `insert` MOVES owned keys and values into the map.
//   • With `String` keys you can still look things up with a `&str`.
//   • Need a stable order? Use `BTreeMap` (quest 6-6) or sort the keys.
//
// 🎯 YOUR TASK
//
//   1. `ledger()`: build the price list: sword 50, shield 30, potion 5.
//   2. Fix `price_of` (0 for unknown items).
//   3. Implement `raise_price`: add `amount` if the item exists and
//      return true; otherwise return false.
//      Tip: `if let Some(p) = map.get_mut(item) { *p += amount; }`
//   4. Implement `cheapest_total`: the sum of all prices.
//      Tip: `for price in map.values() { ... }`

use std::collections::HashMap;

fn ledger() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    // TODO: insert the three items
    map
}

fn price_of(map: &HashMap<String, u32>, item: &str) -> u32 {
    map.get(item)
}

fn raise_price(map: &mut HashMap<String, u32>, item: &str, amount: u32) -> bool {
    todo!()
}

fn cheapest_total(map: &HashMap<String, u32>) -> u32 {
    todo!()
}

fn main() {
    let mut m = ledger();
    raise_price(&mut m, "potion", 2);
    println!("potion costs {}", price_of(&m, "potion"));
    println!("everything costs {}", cheapest_total(&m));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building() {
        let m = ledger();
        assert_eq!(m.len(), 3);
        assert_eq!(m.get("sword"), Some(&50));
    }

    #[test]
    fn prices() {
        let m = ledger();
        assert_eq!(price_of(&m, "shield"), 30);
        assert_eq!(price_of(&m, "dragon"), 0);
    }

    #[test]
    fn raising() {
        let mut m = ledger();
        assert!(raise_price(&mut m, "potion", 2));
        assert_eq!(price_of(&m, "potion"), 7);
        assert!(!raise_price(&mut m, "unicorn", 2));
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn totals() {
        assert_eq!(cheapest_total(&ledger()), 85);
    }
}
