// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Guild Charter                             400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The guild wants a roster library other guilds will use too. It must
//   feel like it came from std: easy to build, easy to loop over, easy
//   to parse and print, with errors that work with `?` everywhere.
//
//   ⚔️  HEAD 1: builder. `Roster::builder().guild("Iron Crabs")
//        .max_size(3).build()`. No guild name (or a blank one) is
//        NoGuildName. Default max_size: 10.
//
//   ⚔️  HEAD 2: `recruit(name)` takes &str or String and returns
//        `Result<&mut Self, RosterError>`, so calls chain with `?`:
//        `r.recruit("Ferris")?.recruit("Lumi")?;`
//        Full → Full { guild, max }. A name already there → AlreadyIn.
//
//   ⚔️  HEAD 3: `for name in &roster` works.
//
//   ⚔️  HEAD 4: text round trip. Display prints
//        "Iron Crabs: Ferris, Lumi" ("Iron Crabs:" when empty), and
//        `"...".parse::<Roster>()` reads it back (max_size 10). No colon,
//        or a blank guild name → BadFormat(the input).
//        `parse()` works for any type with `impl FromStr` (std::str):
//        it calls your `from_str` and returns its Result.
//
//   ⚔️  HEAD 5: RosterError messages (it already implements Error, so
//        `?` turns it into Box<dyn Error>):
//          NoGuildName   "a roster needs a guild name"
//          Full          "{guild} is full ({max} members)"
//          AlreadyIn     "{name} is already in the roster"
//          BadFormat     "can't read a roster from {input:?}"

use std::fmt;
use std::str::FromStr;

pub const DEFAULT_MAX: usize = 10;

#[derive(Debug, PartialEq)]
pub enum RosterError {
    NoGuildName,
    Full { guild: String, max: usize },
    AlreadyIn(String),
    BadFormat(String),
}

impl fmt::Display for RosterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for RosterError {}

#[derive(Debug, PartialEq)]
pub struct Roster {
    guild: String,
    max_size: usize,
    members: Vec<String>,
}

#[derive(Default)]
pub struct RosterBuilder {
    // your fields
}

impl Roster {
    pub fn builder() -> RosterBuilder {
        RosterBuilder::default()
    }

    pub fn guild(&self) -> &str {
        &self.guild
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }
}

impl RosterBuilder {
    // guild, max_size, build
}

impl<'a> IntoIterator for &'a Roster {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl fmt::Display for Roster {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl FromStr for Roster {
    type Err = RosterError;
    fn from_str(s: &str) -> Result<Roster, RosterError> {
        todo!()
    }
}

fn main() {
    println!("{}", RosterError::NoGuildName);
}

// ─── The Charter's tests: don't change these! ────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn head_1_builder() {
        let r = Roster::builder().guild("Iron Crabs").max_size(3).build().unwrap();
        assert_eq!((r.guild(), r.len()), ("Iron Crabs", 0));
        let r = Roster::builder().guild(String::from("Moon Owls")).build().unwrap();
        assert_eq!(r.max_size, DEFAULT_MAX);
        assert_eq!(Roster::builder().max_size(3).build(), Err(RosterError::NoGuildName));
        assert_eq!(Roster::builder().guild("  ").build(), Err(RosterError::NoGuildName));
    }

    #[test]
    fn head_2_recruiting() -> Result<(), Box<dyn Error>> {
        let mut r = Roster::builder().guild("Iron Crabs").max_size(2).build()?;
        r.recruit("Ferris")?.recruit(String::from("Lumi"))?;
        assert_eq!(r.len(), 2);
        assert_eq!(r.recruit("Ada").unwrap_err(), RosterError::Full { guild: "Iron Crabs".into(), max: 2 });
        let mut r = Roster::builder().guild("Moon Owls").build()?;
        r.recruit("Ferris")?;
        assert_eq!(r.recruit("Ferris").unwrap_err(), RosterError::AlreadyIn("Ferris".into()));
        Ok(())
    }

    #[test]
    fn head_3_looping() -> Result<(), Box<dyn Error>> {
        let mut r = Roster::builder().guild("Iron Crabs").build()?;
        r.recruit("Ferris")?.recruit("Lumi")?;
        let mut names = Vec::new();
        for name in &r {
            names.push(name.to_uppercase());
        }
        assert_eq!(names, ["FERRIS", "LUMI"]);
        Ok(())
    }

    #[test]
    fn head_4_text() -> Result<(), Box<dyn Error>> {
        let mut r = Roster::builder().guild("Iron Crabs").build()?;
        assert_eq!(r.to_string(), "Iron Crabs:");
        r.recruit("Ferris")?.recruit("Lumi")?;
        assert_eq!(r.to_string(), "Iron Crabs: Ferris, Lumi");
        let back: Roster = r.to_string().parse()?;
        assert_eq!(back, r);
        let empty: Roster = "Moon Owls:".parse()?;
        assert_eq!((empty.guild(), empty.len()), ("Moon Owls", 0));
        assert_eq!("no colon here".parse::<Roster>(), Err(RosterError::BadFormat("no colon here".into())));
        assert_eq!(" : Ferris".parse::<Roster>(), Err(RosterError::BadFormat(" : Ferris".into())));
        Ok(())
    }

    #[test]
    fn head_5_messages() {
        let full = RosterError::Full { guild: "Iron Crabs".into(), max: 2 };
        assert_eq!(RosterError::NoGuildName.to_string(), "a roster needs a guild name");
        assert_eq!(full.to_string(), "Iron Crabs is full (2 members)");
        assert_eq!(RosterError::AlreadyIn("Lumi".into()).to_string(), "Lumi is already in the roster");
        assert_eq!(RosterError::BadFormat("x".into()).to_string(), "can't read a roster from \"x\"");
        let boxed: Box<dyn Error> = "nope".parse::<Roster>().unwrap_err().into();
        assert_eq!(boxed.to_string(), "can't read a roster from \"nope\"");
    }
}
