# Lesson 10 · The Timeless Library 📚

> Lifetimes: proving that references never outlive their data.

> 💡 **Lifetimes don't change how long things live. They describe it.**
> Annotations are claims that the compiler checks. They never extend or shorten anything.

## Why

```rust
let r;
{ let x = 5; r = &x; }   // ✘ x does not live long enough
println!("{r}");
```

## Annotations

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
// "the result lives as long as BOTH inputs"

fn first_of<'a>(a: &'a str, _b: &str) -> &'a str { a }
// "the result only depends on a"
```

## Elision rules (when you can skip annotations)

| # | Rule | Example |
|---|---|---|
| 1 | each reference input gets its own lifetime | `fn f(a: &str, b: &str)` |
| 2 | exactly one input lifetime → the output gets it | `fn first(s: &str) -> &str` |
| 3 | `&self` present → the output gets self's lifetime | `fn name(&self) -> &str` |

If the rules can't decide, write the lifetimes yourself.

## Structs holding references

```rust
struct Excerpt<'a> { text: &'a str }

impl<'a> Excerpt<'a> {
    // tied to the source, not to self
    fn text(&self) -> &'a str { self.text }
}
fn make(src: &str) -> Excerpt<'_> { Excerpt { text: &src[..5] } }
```

## `'static`

| Form | Meaning |
|---|---|
| `&'static str` | a reference valid for the whole program (literals, statics) |
| `T: 'static` | T holds no short-lived borrows (all owned types qualify) |

## Coming from other languages

- C/C++: lifetimes are the compiler checking what you used to track in your head and in comments.
- GC languages: the GC keeps objects alive as long as they're referenced. Rust instead **rejects** references that might outlive their owner.
- Stuck? Ask "what does this output borrow from?" Or just return an owned type (`String`, `Vec`): a clone is cheap compared to your time.
- Lifetimes on structs are an advanced tool. Most application structs own their data.

## 🎮 Controls

| Key | Action |
|---|---|
| *save the file* | auto re-check |
| `Enter` | re-check now |
| `h` | reveal a hint (that quest gives half XP) |
| `m` | quest map & achievements |
| `q` | save & quit |
| `restore` | put this quest's file back to its starting state (asks first) |
| `reset` | start over: wipe progress and restore all files (asks first) |
