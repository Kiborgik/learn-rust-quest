// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 30-5 · The Procession                            175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Make your collection feel like a std one, and it works with `for`,
//   `.collect()` and `.extend()`. Four small impls:
//
//     // for hero in &party { ... }      (borrowing)
//     impl<'a> IntoIterator for &'a Party {
//         type Item = &'a Hero;
//         type IntoIter = std::slice::Iter<'a, Hero>;
//         fn into_iter(self) -> Self::IntoIter { self.heroes.iter() }
//     }
//
//     // for hero in party { ... }       (consuming: you get owned Heroes)
//     impl IntoIterator for Party { type Item = Hero; type IntoIter = std::vec::IntoIter<Hero>; ... }
//
//     // let party: Party = heroes.into_iter().collect();
//     impl FromIterator<Hero> for Party {
//         fn from_iter<I: IntoIterator<Item = Hero>>(iter: I) -> Self { ... }
//     }
//
//     // party.extend(more_heroes);
//     impl Extend<Hero> for Party { fn extend<I: IntoIterator<Item = Hero>>(&mut self, iter: I) { ... } }
//
//   Plus the usual `iter()` method, returning `impl Iterator<Item = &Hero>`
//   (or the concrete `std::slice::Iter<'_, Hero>`).
//
//   When the inside is a Vec, every one of these is one line: hand the
//   work to the Vec.
//
// 🎯 YOUR TASK
//
//   Add `iter` and the four trait impls so all the tests compile and pass.

#[derive(Debug, Clone, PartialEq)]
pub struct Hero {
    pub name: String,
    pub level: u32,
}

#[derive(Debug, Default, PartialEq)]
pub struct Party {
    heroes: Vec<Hero>,
}

impl Party {
    pub fn len(&self) -> usize {
        self.heroes.len()
    }
}

pub fn hero(name: &str, level: u32) -> Hero {
    Hero { name: name.to_string(), level }
}

fn main() {
    let party = Party::default();
    println!("{} heroes", party.len());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn crew() -> Party {
        vec![hero("Ferris", 12), hero("Lumi", 30), hero("Corro", 3)].into_iter().collect()
    }

    #[test]
    fn collecting_and_extending() {
        let mut party = crew();
        assert_eq!(party.len(), 3);
        party.extend([hero("Ada", 9)]);
        party.extend(crew().into_iter().filter(|h| h.level > 20));
        assert_eq!(party.len(), 5);
    }

    #[test]
    fn looping() {
        let party = crew();
        let mut total = 0;
        for h in &party {
            total += h.level;
        }
        assert_eq!(total, 45);
        let strong: Vec<&str> = party.iter().filter(|h| h.level >= 10).map(|h| h.name.as_str()).collect();
        assert_eq!(strong, ["Ferris", "Lumi"]);
        let names: Vec<String> = party.into_iter().map(|h| h.name).collect();
        assert_eq!(names, ["Ferris", "Lumi", "Corro"]);
    }
}
