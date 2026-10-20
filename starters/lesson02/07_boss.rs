// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Cave Troll                                250 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Cave Troll blocks the exit. It only respects those who master
//   every kind of loop and branch. Write these functions from scratch.
//
//   ⚔️  HEAD 1: `troll_chant(n: u32) -> String`
//        The troll's FizzBuzz: multiples of 3 → "Stomp", multiples of 5
//        → "Roar", multiples of both → "StompRoar", otherwise the number.
//        You need owned text (`String`) here. Lesson 3 explains it. For
//        now just know:  "Stomp".to_string()   and   n.to_string()
//
//   ⚔️  HEAD 2: `collatz_steps(n: u64) -> u32`
//        How many steps to reach 1? Each step: if n is even → n / 2,
//        otherwise → 3 * n + 1.   collatz_steps(6) == 8
//        (Parameters are immutable too. Write `mut n: u64` to change n.)
//
//   ⚔️  HEAD 3: `sum_of_multiples(limit: u32) -> u32`
//        Sum of all numbers BELOW `limit` divisible by 3 or 5.
//        sum_of_multiples(10) == 3 + 5 + 6 + 9 == 23
//
//   ⚔️  HEAD 4: `grade(score: u32) -> char`
//        90..=100 → 'S', 75..=89 → 'A', 50..=74 → 'B', 0..=49 → 'C',
//        anything else → '?'   (a `match` with ranges is perfect)
//
//   💎 BONUS: `first_square_above(n: u32) -> u32`
//        The smallest perfect square STRICTLY greater than n.
//        Try a `loop` that `break`s with a value!
//
//   Your functions go here ↓



fn main() {
    println!("The troll grunts. Run the tests to fight it!");
}

// ─── The Troll's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_1_chant() {
        assert_eq!(troll_chant(1), "1");
        assert_eq!(troll_chant(3), "Stomp");
        assert_eq!(troll_chant(10), "Roar");
        assert_eq!(troll_chant(30), "StompRoar");
        assert_eq!(troll_chant(98), "98");
    }

    #[test]
    fn head_2_collatz() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn head_3_multiples() {
        assert_eq!(sum_of_multiples(10), 23);
        assert_eq!(sum_of_multiples(1000), 233168);
        assert_eq!(sum_of_multiples(1), 0);
    }

    #[test]
    fn head_4_grades() {
        assert_eq!(grade(100), 'S');
        assert_eq!(grade(90), 'S');
        assert_eq!(grade(89), 'A');
        assert_eq!(grade(50), 'B');
        assert_eq!(grade(0), 'C');
        assert_eq!(grade(101), '?');
    }

    #[test]
    fn bonus_squares() {
        assert_eq!(first_square_above(10), 16);
        assert_eq!(first_square_above(16), 25);
        assert_eq!(first_square_above(0), 1);
    }
}
