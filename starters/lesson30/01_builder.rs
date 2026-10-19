// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 30-1 · The Artisan's Order                       175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   This lesson is about API design: making your types pleasant and hard
//   to misuse for the people who call them. Most of it is patterns you've
//   already seen in crates like reqwest, clap and axum.
//
//   A BUILDER helps when a type has many optional settings. Rust has no
//   named or default arguments, so instead of `new(a, b, c, d, e)`:
//
//     let client = Client::builder()
//         .timeout(Duration::from_secs(5))
//         .user_agent("guild/1.0")
//         .build()?;
//
//   Each setter takes `self` by value and returns it, so calls chain:
//
//     pub fn timeout(mut self, t: Duration) -> Self {
//         self.timeout = Some(t);
//         self
//     }
//
//   `build()` checks what's required, fills in defaults, and returns the
//   finished value (or an error).
//
// 🎯 YOUR TASK
//
//   1. The setters can't chain. Make them take and return `self`.
//      `title` should accept both &str and String.
//   2. Write `build`: Err("a quest needs a title") without a title.
//      Defaults: reward 10, min_level 1, not repeatable.

#[derive(Debug, Clone, PartialEq)]
pub struct Quest {
    pub title: String,
    pub reward: u32,
    pub min_level: u32,
    pub repeatable: bool,
}

#[derive(Default)]
pub struct QuestBuilder {
    title: Option<String>,
    reward: Option<u32>,
    min_level: Option<u32>,
    repeatable: bool,
}

impl Quest {
    pub fn builder() -> QuestBuilder {
        QuestBuilder::default()
    }
}

impl QuestBuilder {
    pub fn title(&mut self, title: &str) {
        self.title = Some(title.to_string());
    }

    pub fn reward(&mut self, gold: u32) {
        self.reward = Some(gold);
    }

    pub fn min_level(&mut self, level: u32) {
        self.min_level = Some(level);
    }

    pub fn repeatable(&mut self, yes: bool) {
        self.repeatable = yes;
    }

    pub fn build(self) -> Result<Quest, String> {
        todo!()
    }
}

fn main() {
    let mut b = Quest::builder();
    b.title("Slay the slime");
    println!("{:?}", b.build());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_order() {
        let q = Quest::builder().title("Slay the slime").reward(50).min_level(3).repeatable(true).build();
        assert_eq!(
            q,
            Ok(Quest { title: "Slay the slime".into(), reward: 50, min_level: 3, repeatable: true })
        );
    }

    #[test]
    fn defaults() {
        let title = String::from("Find the cat");
        let q = Quest::builder().title(title).build().unwrap();
        assert_eq!((q.reward, q.min_level, q.repeatable), (10, 1, false));
    }

    #[test]
    fn title_required() {
        assert_eq!(Quest::builder().reward(5).build(), Err("a quest needs a title".to_string()));
    }
}
