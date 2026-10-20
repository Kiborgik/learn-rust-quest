# Changelog

## 0.2.0

Run `git pull` and `cargo run` to update.

- Fixed quests that used stuff from later lessons. The lesson 1 boss
  needed `if` and loops, which only come in lesson 2. Went through all
  30 lessons and fixed the rest.
- Quests you haven't opened yet now update when you pull. Ones you
  already worked on are left alone. If there's a newer version, the
  quest tells you and `restore` gets it. Your old file goes to
  `.quest/backup/`.

## 0.1.0

First release.

- 208 quests in 30 lessons: core Rust (1-12), real crates (serde, clap,
  rayon, tokio, tracing, reqwest, axum, sqlx), a capstone project,
  macros, testing tools, unsafe and FFI, API design.
- Regions and bosses, XP, 23 ranks, achievements, daily streaks, combos
  and a bonus quiz after every quest.
- Checks on every save, with real compiler errors.
- Layout fits the terminal width. Plain mode for screen readers
  (`--plain` or `NO_COLOR=1`).
