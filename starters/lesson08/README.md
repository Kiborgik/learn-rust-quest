# Lesson 8 · The Academy of Forms 🎓

> Traits (shared behaviour) and generics (code for many types).

## Traits

```rust
trait Describe {
    fn name(&self) -> String;                      // required
    fn describe(&self) -> String {                 // default
        format!("This is {}", self.name())
    }
}
impl Describe for Crab { fn name(&self) -> String { "Ferris".into() } }
```

## Generics and bounds

```rust
fn largest<T: PartialOrd + Copy>(v: &[T]) -> T { /* ... */ }
fn show(x: impl Display) {}                       // same idea, shorter
// `where` for long bounds
fn both<T>(x: T) where T: Clone + Debug {}

struct Pair<T> { a: T, b: T }
impl<T> Pair<T> {}            // for all T
impl Pair<f64> {}             // only f64
```

## Static vs dynamic dispatch

| | `impl Trait` / `<T: Trait>` | `dyn Trait` |
|---|---|---|
| Resolved | compile time | runtime (vtable) |
| Mixed types in one Vec | ❌ | ✅ `Vec<Box<dyn Trait>>` |
| Speed | fastest, inlinable | small overhead |
| Return different types | ❌ | ✅ `Box<dyn Trait>` |

## Std traits worth implementing

| Trait | Enables |
|---|---|
| `Display` | `{}`, `.to_string()` |
| `Debug` | `{:?}` (usually derived) |
| `Default` | `T::default()`, `..Default::default()` |
| `From<T>` | `X::from(t)`, `t.into()` |
| `PartialEq`, `Eq` | `==` |
| `PartialOrd`, `Ord` | `<`, `.sort()`, `.max()` |
| `Add`, `Sub`, `Mul` ... | operators |
| `Clone`, `Copy` | copying |
| `Iterator` | `for`, `map`, `filter`... (lesson 9) |

## Coming from other languages

- **No inheritance.** You compose structs and share behaviour with traits and default methods.
- Traits ≈ interfaces, but you can implement them for types you didn't write (`impl MyTrait for i32`).
- **Orphan rule:** you can only `impl Trait for Type` if the trait OR the type is yours, so `impl Display for Vec<T>` is not allowed.
- Generics are monomorphised like C++ templates, but type-checked up front through bounds (like C++20 concepts), not when instantiated.
- Java/C# generics erase types; Rust's don't, so there's no boxing of primitives.

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
