# Lesson 17 · The Town Square 📣

> Command-line apps with `clap`, and how to keep them testable.

```toml
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::{Parser, Subcommand, ValueEnum};

/// Track your inventory              ← about (shown in --help)
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add an item
    Add {
        item: String,
        qty: u32,
        #[arg(long, default_value_t = 0)]
        price: u32,
    },
    List {
        #[arg(long, value_enum, default_value_t = Sort::Name)]
        sort: Sort,
    },
}

#[derive(ValueEnum, Clone)]
enum Sort { Name, Qty }
```

| Field type | Argument |
|---|---|
| `String` (no `#[arg]`) | required positional |
| `bool` + `#[arg(long)]` | flag |
| `Option<T>` | optional |
| `Vec<T>` | repeatable |
| `#[arg(default_value_t = x)]` | has a default |
| `value_parser!(u8).range(1..=10)` | range check |

## Testable CLIs

```rust
fn main() -> anyhow::Result<()> {
    // real args, exits on --help
    let cli = Cli::parse();
    run(cli, &mut std::io::stdout().lock())
}

fn run(
    cli: Cli,
    out: &mut impl std::io::Write,
) -> anyhow::Result<()> { ... }

// test:
let cli = Cli::try_parse_from(["tool", "add", "rope", "3"])?;
let mut buf = Vec::new();
run(cli, &mut buf)?;
```

Keep file and terminal I/O in `main`. Everything else gets plain values.

## Coming from other languages

- Like Python's argparse/click or .NET System.CommandLine, but the types
  ARE the spec: a `u32` field rejects "lots" for you.
- `--help`, `--version`, error messages and suggestions ("did you mean")
  come for free.

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
