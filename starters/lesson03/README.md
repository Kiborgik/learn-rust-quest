# Lesson 3 · The Bridge of Ownership 🌉

> Rust's big idea: memory safety without a garbage collector.

## Stack vs heap (30-second version)

- **Stack**: fixed-size values (`i32`, `bool`, `(u8, char)`, ...). Copying them is cheap.
- **Heap**: growable data (`String`, `Vec`, ...). A small handle on the stack points to it.

Someone has to free heap memory, exactly once. Rust decides who, at compile time.

## The three rules

1. Each value has **one owner**.
2. Ownership can be **moved**.
3. When the owner goes out of scope, the value is **dropped**.

```rust
let a = String::from("sword");
let b = a;            // move: `a` is now unusable
// println!("{a}");   // ✘ borrow of moved value
```

## Copy vs Clone

```rust
let x = 5;  let y = x;               // i32 is Copy: both usable
let s = String::from("hi");
let t = s.clone();                    // explicit deep copy
```

## Borrowing

```rust
fn len(s: &str) -> usize { s.len() }           // shared borrow: read
fn grow(s: &mut String) { s.push('!'); }       // mutable borrow: write

let mut s = String::from("hey");
len(&s);
grow(&mut s);
```

**At any time: many `&T` OR one `&mut T`.** A borrow ends at its last use.

## `String` vs `&str`

| | `String` | `&str` |
|---|---|---|
| Owns data | yes | no (borrowed view) |
| Growable | yes | no |
| Literal `"hi"` | no | yes |
| Use as a parameter | rarely | ✅ preferred |
| Return new text | ✅ | no |

Conversions: `"x".to_string()`, `String::from("x")`, `&my_string` → `&str`.

## Slices

```rust
let s = String::from("hello world");
let w = &s[0..5];           // "hello" (BYTE indexes!)
let a = [1, 2, 3, 4];
let mid: &[i32] = &a[1..3]; // [2, 3]
```

## Coming from other languages

| Language | What `b = a` means | Rust |
|---|---|---|
| Java / C# / Python / JS | two references to one object | **move**: one owner |
| C++ | copy constructor | move; copy only via `.clone()` |
| Go | copy of the header, shared backing data | move |

- There are no null references, and no dangling pointers in safe Rust.
- "Fighting the borrow checker" usually means two parts of the code want to change the same thing at once. Restructure: shorten borrows, return values, or pass `&mut` down.
- `.clone()` is fine while learning. Optimize later.

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
