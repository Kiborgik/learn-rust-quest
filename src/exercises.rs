//! The catalogue: lesson and exercise types, and the list of lessons.
//! Each lesson's content lives in `src/lessons/lessonNN.rs`. See
//! CONTRIBUTING.md for adding one.

use crate::lessons;

pub enum Check {
    /// Compile + run `main`, and compare stdout to this text.
    Output(&'static str),
    /// Compile with `--test` and run the `#[test]`s inside the file.
    Test,
    /// The exercise is a Cargo package (`exercises/lessonNN/<dir>/`):
    /// run `cargo test` (unit, integration and doc tests).
    CargoTest,
    /// Cargo package: `cargo run`, then compare stdout to this text.
    CargoRun(&'static str),
}

impl Check {
    pub fn is_cargo(&self) -> bool {
        matches!(self, Check::CargoTest | Check::CargoRun(_))
    }

    /// Passing means the tests pass (rather than matching output).
    pub fn is_test(&self) -> bool {
        matches!(self, Check::Test | Check::CargoTest)
    }
}

pub struct Quiz {
    pub question: &'static str,
    pub options: &'static [&'static str],
    pub answer: usize,
    pub explain: &'static str,
}

pub struct Exercise {
    /// Unique id, also the `[[bin]]` name in the generated manifest.
    pub id: &'static str,
    pub lesson: u32,
    /// Short label like "2-3", or "BOSS" for the lesson's final fight.
    pub code: &'static str,
    pub title: &'static str,
    pub path: &'static str,
    pub story: &'static str,
    pub check: Check,
    pub xp: u32,
    pub hints: &'static [&'static str],
    pub quiz: Option<Quiz>,
}

impl Exercise {
    pub fn is_boss(&self) -> bool {
        self.code == "BOSS"
    }

    /// "2-3", or "2-BOSS" so bosses of different lessons can be told apart.
    pub fn code_label(&self) -> String {
        if self.is_boss() {
            format!("{}-BOSS", self.lesson)
        } else {
            self.code.to_string()
        }
    }

    /// XP for passing: a revealed hint halves it.
    pub fn reward(&self, used_hint: bool) -> u32 {
        if used_hint { self.xp / 2 } else { self.xp }
    }
}

pub struct Lesson {
    pub number: u32,
    /// "Region: topic", like "The Village of Bindings: variables, types & functions".
    pub title: &'static str,
    pub notes: &'static str,
    /// (achievement id, achievement name) awarded for beating the boss.
    pub boss_trophy: (&'static str, &'static str),
    pub exercises: &'static [Exercise],
}

impl Lesson {
    /// The place name: "The Village of Bindings".
    pub fn region(&self) -> &'static str {
        self.title.split_once(": ").map_or(self.title, |(r, _)| r)
    }

    /// What it teaches: "variables, types & functions" (may be empty).
    pub fn topic(&self) -> &'static str {
        self.title.split_once(": ").map_or("", |(_, t)| t)
    }

    pub fn boss(&self) -> Option<&'static Exercise> {
        self.exercises.iter().find(|e| e.is_boss())
    }
}

pub const LESSONS: &[Lesson] = &[
    lessons::lesson01::LESSON,
    lessons::lesson02::LESSON,
    lessons::lesson03::LESSON,
    lessons::lesson04::LESSON,
    lessons::lesson05::LESSON,
    lessons::lesson06::LESSON,
    lessons::lesson07::LESSON,
    lessons::lesson08::LESSON,
    lessons::lesson09::LESSON,
    lessons::lesson10::LESSON,
    lessons::lesson11::LESSON,
    lessons::lesson12::LESSON,
    lessons::lesson13::LESSON,
    lessons::lesson14::LESSON,
    lessons::lesson15::LESSON,
    lessons::lesson16::LESSON,
    lessons::lesson17::LESSON,
    lessons::lesson18::LESSON,
    lessons::lesson19::LESSON,
    lessons::lesson20::LESSON,
    lessons::lesson21::LESSON,
    lessons::lesson22::LESSON,
    lessons::lesson23::LESSON,
    lessons::lesson24::LESSON,
    lessons::lesson25::LESSON,
    lessons::lesson26::LESSON,
    lessons::lesson27::LESSON,
    lessons::lesson28::LESSON,
    lessons::lesson29::LESSON,
    lessons::lesson30::LESSON,
];

/// Every exercise of every lesson, in play order.
pub fn all() -> impl Iterator<Item = &'static Exercise> {
    LESSONS.iter().flat_map(|l| l.exercises.iter())
}

pub fn lesson(number: u32) -> Option<&'static Lesson> {
    LESSONS.iter().find(|l| l.number == number)
}
