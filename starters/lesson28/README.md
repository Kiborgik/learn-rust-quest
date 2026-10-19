# Lesson 28 · The Tribunal ⚖️

> Testing tools: table tests, property tests, snapshots, fakes and mocks.

```toml
[dev-dependencies]   # tests only, never in your release build
rstest = "0.28"
proptest = "1"
insta = "1"
mockall = "0.15"
```

## Pick the right tool

| Tool | Use it for | Looks like |
|---|---|---|
| `#[test]` | most things | `assert_eq!(f(2), 4)` |
| `rstest` | many cases of one test, shared setup | `#[case(99, Bronze)]`, `#[fixture]` |
| `proptest` | rules true for all inputs | `fn p(x in 0..100u32) { prop_assert!(..) }` |
| `insta` | big or formatted output | `assert_snapshot!(out, @"...")` |
| trait + fake | time, randomness, IO | `Inn::new(FakeClock)` |
| `mockall` | checking calls to a dependency | `expect_withdraw().times(1)` |
| `criterion` | benchmarks (`cargo bench`) | `c.bench_function("sort", ..)` |

## Habits

- Test edges: 0, 1, empty, max, and both sides of every boundary.
- Good properties: round trips, totals that don't change, ranges, "never
  panics".
- Inject what you can't control (clock, RNG, network) behind a trait.
- Mock only calls that matter. Over-mocked tests break on every refactor.
- Snapshots: read the diff before you accept it. `cargo install
  cargo-insta`, then `cargo insta review`.
- `cargo test name_part` runs matching tests only; `-- --nocapture`
  shows println output.

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| JUnit `@ParameterizedTest`, pytest `parametrize` | `rstest` `#[case]` |
| pytest fixtures | `rstest` `#[fixture]` |
| Hypothesis, QuickCheck, fast-check | `proptest` |
| Jest snapshots | `insta` |
| Mockito, Moq, unittest.mock | `mockall` (works on traits) |

## 🎮 Controls

| Key | Action |
|---|---|
| *save the file* | auto re-check |
| `Enter` | re-check now |
| `h` | reveal a hint (that quest gives half XP) |
| `m` | quest map & achievements |
| `q` | save & quit |
| `restore` | put this quest's files back to the start (asks first) |
| `reset` | start over: wipe progress and restore all files (asks first) |
