// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Bazaar Baron                               300 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Bazaar Baron controls every stall in the market. To beat him,
//   run a better shop than he does. Build `Shop` from scratch.
//
//   `struct Shop` should hold:
//     • stock: HashMap<String, u32>   item → quantity
//     • gold:  u32
//     • log:   Vec<String>            history of successful sales
//
//   ⚔️  HEAD 1: `Shop::new(gold: u32) -> Shop` and
//        `restock(&mut self, item: &str, qty: u32)`, which adds to the
//        existing quantity (entry API!). Plus `quantity(&self, item: &str) -> u32`
//        (0 if unknown).
//
//   ⚔️  HEAD 2: `sell(&mut self, item: &str, qty: u32, price_each: u32) -> bool`
//        Fails (returns false, changes nothing) if there isn't enough stock.
//        Otherwise: reduce stock, gold += qty * price_each, push
//        "sold {qty} {item}" to the log, return true.
//        If an item's quantity hits 0, REMOVE it from the stock map.
//
//   ⚔️  HEAD 3: `inventory_report(&self) -> String`
//        Items in ALPHABETICAL order: "apple x3, bread x1"
//        (empty shop → empty string)
//
//   ⚔️  HEAD 4: `best_seller(&self) -> Option<String>`
//        The item that appears in the most log entries (ties: the
//        alphabetically FIRST one). None if nothing was sold.
//        Tip: `entry.split_whitespace()` on "sold 3 apple" gives
//        "sold", "3", "apple". The item is the 3rd word (`.nth(2)`).
//
//   💎 BONUS: `log(&self) -> &[String]` returns the log as a slice.
//
//   Your code goes here ↓

use std::collections::{BTreeMap, HashMap};



fn main() {
    println!("The Baron sneers at your empty stall. Run the tests!");
}

// ─── The Baron's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn shop() -> Shop {
        let mut s = Shop::new(100);
        s.restock("apple", 5);
        s.restock("bread", 2);
        s.restock("apple", 1);
        s
    }

    #[test]
    fn head_1_stock() {
        let s = shop();
        assert_eq!(s.quantity("apple"), 6);
        assert_eq!(s.quantity("bread"), 2);
        assert_eq!(s.quantity("caviar"), 0);
        assert_eq!(s.gold, 100);
    }

    #[test]
    fn head_2_selling() {
        let mut s = shop();
        assert!(s.sell("apple", 4, 3));
        assert_eq!(s.quantity("apple"), 2);
        assert_eq!(s.gold, 112);
        assert!(!s.sell("apple", 10, 3));
        assert_eq!(s.quantity("apple"), 2);
        assert_eq!(s.gold, 112);
        assert!(!s.sell("caviar", 1, 100));
        assert!(s.sell("bread", 2, 5));
        assert!(!s.stock.contains_key("bread"));
    }

    #[test]
    fn head_3_report() {
        let mut s = shop();
        s.restock("cheese", 1);
        assert_eq!(s.inventory_report(), "apple x6, bread x2, cheese x1");
        assert_eq!(Shop::new(0).inventory_report(), "");
    }

    #[test]
    fn head_4_best_seller() {
        let mut s = shop();
        assert_eq!(s.best_seller(), None);
        s.sell("bread", 1, 1);
        s.sell("apple", 1, 1);
        s.sell("apple", 1, 1);
        s.sell("bread", 1, 1);
        assert_eq!(s.best_seller(), Some(String::from("apple")));
        s.sell("apple", 1, 1);
        assert_eq!(s.best_seller(), Some(String::from("apple")));
    }

    #[test]
    fn bonus_log() {
        let mut s = shop();
        s.sell("apple", 2, 1);
        assert_eq!(s.log(), &[String::from("sold 2 apple")]);
    }
}
