# Lesson 30 · The Guild Charter 📜

> API design: builders, typestate, flexible arguments, `Cow`, iterator
> traits, extension traits.

Code is read and called far more than it's written. These patterns make
your types feel like std's.

## Patterns

| Pattern | When | Shape |
|---|---|---|
| builder | many optional settings | `X::builder().a(1).b(2).build()?` |
| typestate | some methods only valid in some states | `Door<Locked>` / `Door<Open>` + `PhantomData` |
| `impl Into<String>` | you store the argument | `fn new(name: impl Into<String>)` |
| `impl AsRef<str>` / `AsRef<Path>` | you only read it | `fn has(&self, n: impl AsRef<str>)` |
| `impl IntoIterator<Item = T>` | you loop over it | arrays, Vecs, iterators all work |
| `Cow<'_, str>` | usually unchanged, sometimes new | `Cow::Borrowed` / `Cow::Owned` |
| `IntoIterator for &T`, `FromIterator`, `Extend` | collections | `for x in &c`, `.collect()`, `.extend()` |
| extension trait | methods on types you don't own | `trait FooExt`, blanket `impl<I: Iterator> ...` |
| `Display` + `FromStr` | text round trips | `x.to_string()`, `s.parse::<X>()?` |
| `impl Error` | your error type | works with `?` and `Box<dyn Error>` |

## A checklist for public types

- Derive what's cheap and useful: `Debug`, `Clone`, `PartialEq`, `Default`.
- Errors implement `std::error::Error`; messages are lowercase, no period.
- Take `&self` when you can, `self` when the call consumes or transforms.
- Don't panic on bad input in a library; return `Result`.
- Read the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| named / default arguments | builder, or `..Default::default()` |
| state machines with enums + checks | typestate (checks at compile time) |
| overloading on string types | `impl Into<String>`, `AsRef<str>` |
| C# extension methods, Kotlin extensions | extension traits |
| `IEnumerable` / `Iterable` | `IntoIterator`, `FromIterator` |

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
