//! Saves XP, finished exercises, hints, achievements and the daily streak to
//! `.quest/progress.txt` in a simple `key=value` format.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default)]
pub struct Progress {
    pub xp: u32,
    pub done: BTreeSet<String>,
    pub hints_used: BTreeSet<String>,
    pub quizzes_passed: BTreeSet<String>,
    pub quizzes_answered: BTreeSet<String>,
    pub achievements: BTreeSet<String>,
    /// Quests passed on the first check without a hint.
    pub first_try: BTreeSet<String>,
    /// Last day played, in days since 1970 (UTC).
    pub day: u64,
    /// Days in a row played, ending on `day`.
    pub streak: u32,
    pub best_streak: u32,
    /// Day the daily bonus was last paid.
    pub bonus_day: u64,
}

fn file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".quest")
        .join("progress.txt")
}

fn today() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() / 86_400)
}

impl Progress {
    /// Every saved set with its key in the save file. Load and save both use
    /// this, so a new set only needs a field and a line here.
    fn sets(&mut self) -> [(&'static str, &mut BTreeSet<String>); 6] {
        [
            ("done", &mut self.done),
            ("hint", &mut self.hints_used),
            ("quiz", &mut self.quizzes_passed),
            ("answered", &mut self.quizzes_answered),
            ("achievement", &mut self.achievements),
            ("first_try", &mut self.first_try),
        ]
    }

    /// True if a save file exists (false on the very first run).
    pub fn exists() -> bool {
        file().is_file()
    }

    pub fn load() -> Self {
        let mut p = Progress::default();
        let Ok(text) = fs::read_to_string(file()) else {
            return p;
        };
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let (key, value) = (key.trim(), value.trim());
            match key {
                "xp" => p.xp = value.parse().unwrap_or(0),
                "day" => p.day = value.parse().unwrap_or(0),
                "streak" => p.streak = value.parse().unwrap_or(0),
                "best_streak" => p.best_streak = value.parse().unwrap_or(0),
                "bonus_day" => p.bonus_day = value.parse().unwrap_or(0),
                _ => {
                    if let Some((_, set)) = p.sets().into_iter().find(|(k, _)| *k == key) {
                        set.insert(value.to_string());
                    }
                }
            }
        }
        p
    }

    /// Records that the game was played today and updates the streak.
    /// Returns true if the streak grew (first play of a new day in a row).
    pub fn touch_day(&mut self) -> bool {
        let now = today();
        if self.day == now && self.streak > 0 {
            return false;
        }
        self.streak = if self.day + 1 == now {
            self.streak + 1
        } else {
            1
        };
        self.day = now;
        self.best_streak = self.best_streak.max(self.streak);
        self.streak > 1
    }

    /// True once per day: the first quest of the day earns a bonus.
    pub fn claim_daily_bonus(&mut self) -> bool {
        let now = today();
        let fresh = self.bonus_day != now;
        self.bonus_day = now;
        fresh
    }

    pub fn save(&mut self) {
        let mut out = format!(
            "xp={}\nday={}\nstreak={}\nbest_streak={}\nbonus_day={}\n",
            self.xp, self.day, self.streak, self.best_streak, self.bonus_day
        );
        for (key, set) in self.sets() {
            for v in set.iter() {
                out.push_str(&format!("{key}={v}\n"));
            }
        }
        let path = file();
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Err(e) = fs::write(&path, out) {
            eprintln!("warning: could not save progress: {e}");
        }
    }

    pub fn reset() {
        let _ = fs::remove_file(file());
    }
}
