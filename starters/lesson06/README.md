# Lesson 6 · The Merchant's Bazaar 🛒

> `Vec`, `String` (again), `HashMap`, sets, and the B-tree versions.

## Vec

```rust
let mut v = vec![1, 2, 3];
v.push(4);
let last = v.pop();          // Option<i32>
let x = v[0];                // panics if out of range
let y = v.get(10);           // Option<&i32>
for n in &v {}               // read
for n in &mut v { *n += 1 }  // modify
for n in v {}                // consume
```

## String

```rust
let mut s = String::from("crab");
s.push_str("cake"); s.push('!');
s.len();                // BYTES
s.chars().count();      // characters
s.chars().next();       // first char (Option<char>)
for w in s.split_whitespace() {}
```

There is no `s[i]`, because UTF-8 characters vary in size.

## HashMap and friends

```rust
use std::collections::HashMap;
let mut m: HashMap<String, u32> = HashMap::new();
m.insert("a".into(), 1);
m.get("a");                              // Option<&u32>
*m.entry("b".into()).or_insert(0) += 1;  // count / upsert
if let Some(v) = m.get_mut("a") { *v += 1; }
```

| Need | Type |
|---|---|
| ordered list | `Vec<T>` |
| key → value, fast | `HashMap<K, V>` |
| key → value, sorted | `BTreeMap<K, V>` |
| unique values | `HashSet<T>` |
| unique and sorted | `BTreeSet<T>` |
| queue / deque | `VecDeque<T>` |

## Coming from other languages

- `Vec<T>` holds **one type**. For mixed values, use an enum (lesson 5) or trait objects (lesson 8).
- **HashMap order is random.** Never depend on it, and use `BTreeMap` for sorted output.
- Indexing out of bounds **panics**, but it's never undefined behaviour. Use `.get()` when an index might be missing.
- The borrow checker prevents changing a collection while you hold references into it. This is the bug class Java reports as `ConcurrentModificationException`.
- Sneak peek: `v.iter().map(|x| x * 2).filter(...).collect()` arrives in lesson 9.

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
