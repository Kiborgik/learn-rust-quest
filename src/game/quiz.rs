//! The one-shot bonus quiz after a quest.

use super::Game;
use crate::exercises::{Exercise, Quiz};
use crate::rewards::QUIZ_XP;
use crate::ui::*;

const LETTERS: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

impl Game {
    /// Asks the quiz and pays out. Returns false if input was closed.
    pub(super) fn quiz(&mut self, ex: &Exercise, quiz: &Quiz) -> bool {
        println!();
        rule();
        println!(
            "  {}  {}",
            bold(cyan("🧠 BONUS QUIZ")),
            dim(format!("+{QUIZ_XP} XP, one shot!"))
        );
        println!();
        for line in quiz.question.lines() {
            println!("  {line}");
        }
        println!();
        // Display order -> original option index. Shuffled so the correct
        // answer isn't always in the same slot.
        let order = shuffled_order(ex.id, quiz.options.len());
        let correct = order.iter().position(|&o| o == quiz.answer).unwrap_or(0);
        for (i, &o) in order.iter().enumerate() {
            for (j, line) in wrap(quiz.options[o], cols().saturating_sub(6))
                .iter()
                .enumerate()
            {
                let tag = if j == 0 {
                    format!("{})", LETTERS[i])
                } else {
                    "  ".into()
                };
                println!("  {} {line}", bold(tag));
            }
        }

        let Some(picked) = self.ask_letter(quiz.options.len()) else {
            return false;
        };

        self.progress.quizzes_answered.insert(ex.id.to_string());
        if picked == correct {
            self.progress.xp += QUIZ_XP;
            self.progress.quizzes_passed.insert(ex.id.to_string());
            println!(
                "\n  {} {}",
                bold(green("✔ Correct!")),
                yellow(format!("+{QUIZ_XP} XP"))
            );
        } else {
            println!(
                "\n  {} The answer was {}).",
                bold(red("✘ Not quite.")),
                LETTERS[correct]
            );
        }
        print_wrapped(2, quiz.explain, dim);
        rule();
        self.progress.save();
        true
    }

    /// Reads a letter for one of `n` options. None if input was closed.
    fn ask_letter(&self, n: usize) -> Option<usize> {
        loop {
            print!("\n  Your answer: ");
            flush();
            let line = self.input.recv().ok()?.trim().to_lowercase();
            if let Some(i) = LETTERS[..n].iter().position(|l| *l == line) {
                return Some(i);
            }
            print!(
                "  {}",
                dim(format!("Type a letter from a to {}.", LETTERS[n - 1]))
            );
        }
    }
}

/// Deterministic Fisher-Yates shuffle of `0..n`, seeded by the exercise id
/// (FNV-1a + xorshift), so a quiz always shows the same order.
fn shuffled_order(id: &str, n: usize) -> Vec<usize> {
    let mut state = crate::versions::fnv1a(id.as_bytes()) | 1;
    let mut order: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        order.swap(i, (state % (i as u64 + 1)) as usize);
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_is_a_stable_permutation() {
        let a = shuffled_order("l1_01_hello", 4);
        assert_eq!(a, shuffled_order("l1_01_hello", 4));
        let mut sorted = a.clone();
        sorted.sort();
        assert_eq!(sorted, [0, 1, 2, 3]);
    }
}
