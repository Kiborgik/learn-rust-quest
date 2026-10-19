// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 30-3 · Open Hands                                175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Accept what's convenient for the caller, so they don't litter their
//   code with `.to_string()` and `vec![...]`:
//
//     fn new(name: impl Into<String>)        // &str, String, Box<str>, ...
//     fn has(&self, name: impl AsRef<str>)   // anything that can lend a &str
//     fn open(path: impl AsRef<Path>)        // &str, String, PathBuf, &Path
//     fn add_all(names: impl IntoIterator<Item = impl Into<String>>)
//                                            // arrays, Vecs, iterators...
//
//   Rules of thumb:
//     - you'll STORE it as String  → `impl Into<String>`, call `.into()`
//     - you only READ it           → `impl AsRef<str>` (or plain &str)
//     - you'll loop over it        → `impl IntoIterator<Item = ...>`
//
//   Std does this everywhere: `File::open(impl AsRef<Path>)`,
//   `Vec::extend(impl IntoIterator)`, `HashMap::from([(k, v), ...])`.
//
//   (Don't overdo it. Each generic parameter makes a new copy of the
//   function and can make errors harder to read. Use it on public APIs
//   people call a lot.)
//
// 🎯 YOUR TASK
//
//   Change the signatures (and bodies) so every call in the tests works.

pub struct Party {
    name: String,
    members: Vec<String>,
}

impl Party {
    pub fn new(name: String) -> Party {
        Party { name, members: Vec::new() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add(&mut self, hero: String) {
        self.members.push(hero);
    }

    pub fn add_all(&mut self, heroes: Vec<String>) {
        for h in heroes {
            self.add(h);
        }
    }

    pub fn has(&self, hero: &String) -> bool {
        self.members.iter().any(|m| m == hero)
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }
}

fn main() {
    let mut p = Party::new(String::from("Iron Crabs"));
    p.add(String::from("Ferris"));
    println!("{} has {} members", p.name(), p.len());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_calls() {
        let mut p = Party::new("Iron Crabs");
        let corro = String::from("Corro");
        p.add("Ferris");
        p.add(corro.clone());
        p.add_all(["Lumi", "Ada"]);
        p.add_all(vec![String::from("Zed")]);
        p.add_all(["x", "y"].iter().map(|s| s.to_uppercase()));
        assert_eq!(p.len(), 7);
        assert!(p.has("Lumi"));
        assert!(p.has(&corro));
        assert!(p.has(String::from("X")));
        assert!(!p.has("Bob"));
    }

    #[test]
    fn owned_names_too() {
        let p = Party::new(String::from("Moon Owls"));
        assert_eq!(p.name(), "Moon Owls");
    }
}
