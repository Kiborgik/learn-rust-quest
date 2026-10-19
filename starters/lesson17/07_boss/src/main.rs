// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Ledger Lich                               400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   Your first real tool: `ledger`, an inventory tracker for adventurers.
//   It uses everything from lessons 13 to 17: clap, serde, anyhow and
//   itertools. The Lich will audit every command.
//
//   The ledger is saved as JSON. `run` gets the arguments, the current
//   JSON state ("" means a new, empty ledger) and an output writer. It
//   returns the new JSON state. `main` (already written) wires it to a
//   real file.
//
//   Commands:
//
//     ledger add <item> <qty> [--price <gold>]   price per item, default 0
//         → "added 3 rope"       (adding again adds qty; the new price wins)
//     ledger remove <item> <qty>
//         → "removed 1 rope"
//         error "not enough rope (have 2)" if qty is too big
//         error "no item rope" if it isn't in the ledger
//         an item at 0 is removed from the ledger
//     ledger list [--sort name|qty]               default name; qty = highest first,
//         one line per item: "rope x3 @2"           ties by name
//         "(empty)" for an empty ledger
//     ledger total
//         → "total: 42 gold"     sum of qty * price
//
//   ⚔️  HEAD 1: the clap types (`Cli`, `Command`, a `Sort` ValueEnum)
//   ⚔️  HEAD 2: `Ledger` with Serialize/Deserialize (BTreeMap keeps it tidy)
//   ⚔️  HEAD 3: add / remove with anyhow errors
//   ⚔️  HEAD 4: list with both sort orders (itertools helps)
//   ⚔️  HEAD 5: total, plus the JSON round trip between calls
//
//   Bad arguments (unknown command, "--sort price", "add rope lots") must
//   come back as errors from `run`, not panics: use `Cli::try_parse_from`.

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Write;

// Your types go here ↓


pub fn run(args: &[&str], state: &str, out: &mut impl Write) -> Result<String> {
    todo!()
}

fn main() -> Result<()> {
    let path = "ledger.json";
    let state = std::fs::read_to_string(path).unwrap_or_default();
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let new_state = run(&args, &state, &mut std::io::stdout().lock())?;
    std::fs::write(path, new_state).context("saving ledger.json")
}

// ─── The Lich's tests: don't change these! ───────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs a sequence of commands, returning (all output, final state).
    fn session(cmds: &[&str]) -> (String, String) {
        let mut state = String::new();
        let mut out = Vec::new();
        for cmd in cmds {
            let args: Vec<&str> = std::iter::once("ledger").chain(cmd.split_whitespace()).collect();
            state = run(&args, &state, &mut out).unwrap();
        }
        (String::from_utf8(out).unwrap(), state)
    }

    fn fails(state_cmds: &[&str], cmd: &str) -> String {
        let (_, state) = session(state_cmds);
        let args: Vec<&str> = std::iter::once("ledger").chain(cmd.split_whitespace()).collect();
        format!("{:#}", run(&args, &state, &mut Vec::new()).unwrap_err())
    }

    #[test]
    fn head_1_parsing_errors() {
        assert!(fails(&[], "dance").contains("unrecognized subcommand"));
        assert!(fails(&[], "list --sort price").contains("invalid value"));
        assert!(fails(&[], "add rope lots").contains("invalid value"));
    }

    #[test]
    fn head_2_state_is_json() {
        let (_, state) = session(&["add rope 3 --price 2"]);
        let v: serde_json::Value = serde_json::from_str(&state).unwrap();
        assert!(v.is_object(), "{state}");
        assert!(state.contains("rope"));
    }

    #[test]
    fn head_3_add_remove() {
        let (out, _) = session(&["add rope 3 --price 2", "add rope 2", "remove rope 4"]);
        assert_eq!(out, "added 3 rope\nadded 2 rope\nremoved 4 rope\n");
        assert_eq!(fails(&["add rope 2"], "remove rope 5"), "not enough rope (have 2)");
        assert_eq!(fails(&[], "remove rope 1"), "no item rope");
        let (out, _) = session(&["add rope 1", "remove rope 1", "list"]);
        assert!(out.ends_with("(empty)\n"), "{out}");
    }

    #[test]
    fn head_4_listing() {
        let setup = ["add torch 1 --price 5", "add rope 3 --price 2", "add apple 3"];
        let (out, _) = session(&[&setup[..], &["list"]].concat());
        assert!(out.ends_with("apple x3 @0\nrope x3 @2\ntorch x1 @5\n"), "{out}");
        let (out, _) = session(&[&setup[..], &["list --sort qty"]].concat());
        assert!(out.ends_with("apple x3 @0\nrope x3 @2\ntorch x1 @5\n"), "{out}");
        let (out, _) = session(&["add a 1", "add b 5", "add c 2", "list --sort qty"]);
        assert!(out.ends_with("b x5 @0\nc x2 @0\na x1 @0\n"), "{out}");
    }

    #[test]
    fn head_5_total() {
        let (out, _) = session(&["add torch 2 --price 5", "add rope 3 --price 4", "total"]);
        assert!(out.ends_with("total: 22 gold\n"), "{out}");
        let (out, _) = session(&["total"]);
        assert_eq!(out, "total: 0 gold\n");
    }
}
