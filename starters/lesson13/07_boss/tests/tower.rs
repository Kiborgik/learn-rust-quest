// ─── The Tower's tests: don't change these! ──────────────────────────────────

use l13_07_boss::dice::Die;
use l13_07_boss::notation::{ParseError, parse};
use l13_07_boss::tower::Tower;
use rand::{SeedableRng, rngs::StdRng};

#[test]
fn head_2_dice() {
    assert!(Die::new(1).is_none());
    assert!(Die::new(0).is_none());
    let d = Die::new(8).unwrap();
    assert_eq!(d.sides(), 8);
    let mut rng = StdRng::seed_from_u64(3);
    let mut seen = [false; 9];
    for _ in 0..5_000 {
        let n = d.roll(&mut rng);
        assert!((1..=8).contains(&n));
        seen[n as usize] = true;
    }
    assert_eq!(seen[1..], [true; 8]);
}

#[test]
fn head_3_parse_ok() {
    let r = parse("2d6+3").unwrap();
    assert_eq!((r.count, r.die.sides(), r.bonus), (2, 6, 3));
    let r = parse(" d20 ").unwrap();
    assert_eq!((r.count, r.die.sides(), r.bonus), (1, 20, 0));
    let r = parse("1d4-1").unwrap();
    assert_eq!((r.count, r.die.sides(), r.bonus), (1, 4, -1));
}

#[test]
fn head_3_parse_errors() {
    assert_eq!(parse("   "), Err(ParseError::Empty));
    assert_eq!(parse("26"), Err(ParseError::MissingD));
    assert_eq!(parse("xd6"), Err(ParseError::BadNumber("x".into())));
    assert_eq!(parse("0d6"), Err(ParseError::BadNumber("0".into())));
    assert_eq!(parse("2dz"), Err(ParseError::BadNumber("z".into())));
    assert_eq!(parse("2d6+q"), Err(ParseError::BadNumber("q".into())));
    assert_eq!(parse("2d1"), Err(ParseError::BadDie(1)));
}

#[test]
fn head_3_ranges_and_display() {
    let r = parse("2d6+3").unwrap();
    assert_eq!((r.min(), r.max()), (5, 15));
    let mut rng = StdRng::seed_from_u64(9);
    for _ in 0..2_000 {
        let n = r.roll(&mut rng);
        assert!((5..=15).contains(&n), "rolled {n}");
    }
    assert_eq!(r.to_string(), "2d6+3");
    assert_eq!(parse("d20").unwrap().to_string(), "1d20");
    assert_eq!(parse("1d4-1").unwrap().to_string(), "1d4-1");
}

#[test]
fn head_4_tower() {
    let t = Tower::parse("2d6+3, d20,1d4-1").unwrap();
    assert_eq!(t.rolls.len(), 3);
    assert_eq!(t.max_total(), 15 + 20 + 3);
    let mut rng = StdRng::seed_from_u64(5);
    let results = t.roll_all(&mut rng);
    assert_eq!(results.len(), 3);
    assert!((5..=15).contains(&results[0]));
    assert!((1..=20).contains(&results[1]));
    assert!((0..=3).contains(&results[2]));
    assert_eq!(Tower::parse("d6, 2x8").unwrap_err(), ParseError::MissingD);
}
