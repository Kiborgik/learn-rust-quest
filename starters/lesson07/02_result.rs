// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 7-2 · Two Paths                                  125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A RECOVERABLE error is just a return value of this std enum:
//
//     enum Result<T, E> {
//         Ok(T),        // success, carrying the value
//         Err(E),       // failure, carrying the error
//     }
//
//     fn divide(a: u32, b: u32) -> Result<u32, String> {
//         if b == 0 {
//             return Err(String::from("division by zero"));
//         }
//         Ok(a / b)
//     }
//
//     match divide(10, 2) {
//         Ok(v) => println!("{v}"),
//         Err(e) => println!("oops: {e}"),
//     }
//
//   Parsing text returns a Result too:
//
//     let n: Result<u32, _> = "42".parse();        // Ok(42)
//     let bad: Result<u32, _> = "abc".parse();     // Err(ParseIntError)
//     "42".parse::<u32>();                          // "turbofish" syntax
//
//   Like Go's `(value, err)`, but you CAN'T forget to check: the value
//   is locked inside `Ok`. Unlike exceptions, failure is visible in the
//   function's signature.
//
// 🎯 YOUR TASK
//
//   1. Fix `buy_potion` (it doesn't compile).
//   2. Implement `parse_level`: parse the text as a u32.
//      Not a number → Err("not a number"), 0 → Err("level must be at least 1")
//      Tip: `match text.parse::<u32>() { Ok(n) => ..., Err(_) => ... }`

fn buy_potion(gold: u32, price: u32) -> Result<u32, String> {
    if gold < price {
        Err(format!("need {} more gold", price - gold))
    }
    gold - price
}

fn parse_level(text: &str) -> Result<u32, String> {
    todo!()
}

fn main() {
    match buy_potion(10, 25) {
        Ok(left) => println!("Bought! {left} gold left"),
        Err(e) => println!("Can't buy: {e}"),
    }
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buying() {
        assert_eq!(buy_potion(30, 25), Ok(5));
        assert_eq!(buy_potion(10, 25), Err(String::from("need 15 more gold")));
    }

    #[test]
    fn levels() {
        assert_eq!(parse_level("12"), Ok(12));
        assert_eq!(parse_level("twelve"), Err(String::from("not a number")));
        assert_eq!(parse_level("0"), Err(String::from("level must be at least 1")));
    }
}
