// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Fortress Gatekeeper                        300 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Fortress of Failure only opens for travellers with a perfect
//   gate pass. The Gatekeeper rejects bad passes, and it tells you
//   EXACTLY what's wrong with them.
//
//   A pass looks like:   "name=Ferris;level=12;guild=Crabs"
//   (fields separated by ';', each one "key=value", in any order)
//
//   Write:
//
//     #[derive(Debug, PartialEq)]
//     struct Pass { name: String, level: u32, guild: String }
//
//     #[derive(Debug, PartialEq)]
//     enum PassError {
//         Malformed(String),          // a field without '=' (holds that field)
//         MissingField(&'static str), // "name", "level" or "guild"
//         BadLevel(ParseIntError),    // level isn't a number
//         TooLowLevel { level: u32, required: u32 },
//     }
//
//   ⚔️  HEAD 1: `parse_pass(text: &str) -> Result<Pass, PassError>`
//        Parse a correct pass. Unknown keys are ignored.
//
//   ⚔️  HEAD 2: errors: Malformed and MissingField
//        (check fields in this order: name, level, guild)
//
//   ⚔️  HEAD 3: `impl From<ParseIntError> for PassError` so a bad level
//        becomes BadLevel through `?`
//
//   ⚔️  HEAD 4: `enter(text: &str) -> Result<String, PassError>`
//        Parses the pass. Level must be at least 10, otherwise
//        TooLowLevel { level, required: 10 }.
//        Success: "Welcome, Ferris of Crabs!"
//
//   ⚔️  HEAD 5: `impl Display` for PassError:
//        Malformed(f)         → "malformed field: {f}"
//        MissingField(f)      → "missing field: {f}"
//        BadLevel(e)          → "bad level: {e}"
//        TooLowLevel{..}      → "level {level} is below {required}"
//
//   Tip: keep `Option<String>` variables for each field while scanning,
//   then turn each one into a Result with `.ok_or(...)?`.
//
//   Your code goes here ↓

use std::fmt;
use std::num::ParseIntError;



fn main() {
    println!("The Gatekeeper raises a gauntlet. Run the tests!");
}

// ─── The Gatekeeper's tests: don't change these! ─────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_1_valid_pass() {
        let p = parse_pass("name=Ferris;level=12;guild=Crabs").unwrap();
        assert_eq!(
            p,
            Pass { name: "Ferris".into(), level: 12, guild: "Crabs".into() }
        );
        let q = parse_pass("guild=Owls;color=blue;level=3;name=Hoot").unwrap();
        assert_eq!(q.name, "Hoot");
        assert_eq!(q.level, 3);
    }

    #[test]
    fn head_2_shape_errors() {
        assert_eq!(
            parse_pass("name=Ferris;oops;guild=Crabs"),
            Err(PassError::Malformed("oops".into()))
        );
        assert_eq!(
            parse_pass("level=12;guild=Crabs"),
            Err(PassError::MissingField("name"))
        );
        assert_eq!(
            parse_pass("name=Ferris;level=12"),
            Err(PassError::MissingField("guild"))
        );
    }

    #[test]
    fn head_3_bad_level() {
        let err = parse_pass("name=Ferris;level=twelve;guild=Crabs").unwrap_err();
        assert!(matches!(err, PassError::BadLevel(_)));
    }

    #[test]
    fn head_4_entering() {
        assert_eq!(
            enter("name=Ferris;level=12;guild=Crabs"),
            Ok(String::from("Welcome, Ferris of Crabs!"))
        );
        assert_eq!(
            enter("name=Tiny;level=2;guild=Crabs"),
            Err(PassError::TooLowLevel { level: 2, required: 10 })
        );
    }

    #[test]
    fn head_5_messages() {
        assert_eq!(PassError::Malformed("x".into()).to_string(), "malformed field: x");
        assert_eq!(PassError::MissingField("name").to_string(), "missing field: name");
        assert_eq!(
            PassError::TooLowLevel { level: 2, required: 10 }.to_string(),
            "level 2 is below 10"
        );
        let err = parse_pass("name=F;level=;guild=C").unwrap_err();
        assert_eq!(err.to_string(), "bad level: cannot parse integer from empty string");
    }
}
