# Contributing

Thanks for helping. Fixes to unclear or broken quests are the most useful
contributions.

## Setup

```sh
cargo run                                  # play
cargo test                                 # quick checks
# also every starter (slow the first time)
cargo test -- --include-ignored
cargo fmt && cargo clippy --all-targets -- -D warnings
```

The game uses only the standard library. Please keep it that way.

## How it works

- `src/lessons/lessonNN.rs`: the catalogue for a lesson (stories, hints, quizzes).
- `src/exercises.rs`: catalogue types and the `LESSONS` list.
- `src/rewards.rs`: XP numbers, ranks and achievements, as data. Tune the
  game's balance here.
- `src/game/`: the game loop: playing a quest, finishing it, the quiz,
  in-game commands (one `COMMANDS` table), reset.
- `src/view/`: everything on screen: header and quest card, banners and
  flavor text, the world map.
- `src/ui/`: the terminal toolkit: colors, width-aware layout, small
  animations, plain mode, and the per-OS terminal calls.
- `src/runner.rs`: compiles one exercise with `rustc` or `cargo` and checks it.
- `src/workspace.rs`: copies `starters/` to `exercises/`, `restore`, `reset`.
- `src/progress.rs`: the save file in `.quest/`.
- `starters/lessonNN/`: original exercise files and the lesson's `README.md`.
  This is the only place exercises live in git.

Play order is the order of `LESSONS`. Lesson numbers are part of saved ids
(`l3_04_mut_borrow`), so never renumber a published lesson. New lessons get
the next free number.

## Exercise rules

- 7 per lesson: `N-1` to `N-6` and a `BOSS` (test-driven, mostly empty body).
- The id `lN_0K_topic` matches the file name and the catalogue `id`.
- Every file has `fn main()`.
- Each file starts with a `📖 LEARN BY EXAMPLE` block, then `🎯 YOUR TASK`,
  then the broken code.
- It must **fail as shipped** and pass after a small fix that the comments
  teach. Use `todo!()` for functions to write.
- `Check::Output` compares trimmed stdout, so output must be deterministic
  (no `HashMap` order, no thread races). `Check::Test` runs the file's tests;
  mark them `// ─── Tests: don't change these! ───`.
- Give each quest 2 to 4 hints (the last one nearly gives the answer) and one
  4-option quiz. Options are shuffled, so avoid "all of the above".
- Each lesson `README.md` has short notes, a "Coming from other languages"
  section and the controls table.

## Workshop (Cargo) exercises

From lesson 13, a quest can be a whole Cargo package instead of one file:

- Put it in `starters/lessonNN/<NN_topic>/` with its own `Cargo.toml`
  (package name = the exercise id, edition 2024) and `src/`, `tests/` as needed.
- In the catalogue, `path` is the file to open first, and `check` is
  `Check::CargoTest` (runs `cargo test`) or `Check::CargoRun("expected output")`.
- The game copies the whole package and adds it as a member of the generated
  `exercises/Cargo.toml` workspace (shared `target/` and `Cargo.lock`).
- `restore` puts every file back and moves files the player added to
  `.quest/backup/`.
- Builds get a long timeout; only the run counts against the normal limit.
- Checks must never start a real server or use the internet. Downloading
  crates on the first build is fine.

## Adding a lesson

1. Write the files and `README.md` in `starters/lessonNN/`.
2. Add `src/lessons/lessonNN.rs`, register it in `src/lessons/mod.rs` and in
   `LESSONS` in `src/exercises.rs`.
3. Extend `RANKS` if the new XP goes past the top rank.
4. Run `cargo test -- --include-ignored`. It checks that every starter exists
   and fails as shipped.
5. Solve each exercise yourself in a scratch copy to make sure it can pass.
   Don't commit solutions.

## Writing style

Short sentences, plain English, concrete examples. Compare to other languages
where it helps.
