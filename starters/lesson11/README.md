# Lesson 11 · The Clockwork Tower ⚙️

> `Box`, `Deref`/`Drop`, `Rc`, `Cell`/`RefCell`, `Weak`.

## Which pointer do I need?

| Need | Use |
|---|---|
| a value on the heap / a recursive type / `dyn Trait` | `Box<T>` |
| several owners, single thread | `Rc<T>` |
| several owners, many threads | `Arc<T>` (lesson 12) |
| mutate a Copy value through `&self` | `Cell<T>` |
| mutate anything through `&self` (runtime-checked) | `RefCell<T>` |
| shared AND mutable, single thread | `Rc<RefCell<T>>` |
| shared AND mutable, many threads | `Arc<Mutex<T>>` (lesson 12) |
| a back-pointer that shouldn't own | `Weak<T>` |

## Box

```rust
enum List { Cons(i32, Box<List>), Nil }
let shapes: Vec<Box<dyn Shape>> =
    vec![Box::new(Circle), Box::new(Square)];
```

## Deref and Drop

```rust
impl Deref for Wrapper {
    type Target = String;
    fn deref(&self) -> &String { &self.0 }
}
impl Drop for Wrapper { fn drop(&mut self) { println!("cleanup!"); } }
drop(x); // drop early
```

Locals drop in **reverse declaration order**. `let _ = x()` drops immediately; `let _guard = x()` lives until the end of the scope.

## Rc, RefCell and Weak

```rust
let shared = Rc::new(RefCell::new(vec![1]));
let other = Rc::clone(&shared);
other.borrow_mut().push(2);
Rc::strong_count(&shared);           // 2

let w = Rc::downgrade(&shared);      // Weak
w.upgrade();                         // Option<Rc<..>>
```

`RefCell` breaking the borrow rules → a **panic** at runtime, not a compile error.

## Coming from other languages

| C++ | Rust |
|---|---|
| `unique_ptr<T>` | `Box<T>` |
| `shared_ptr<T>` | `Rc<T>` / `Arc<T>` |
| `weak_ptr<T>` | `Weak<T>` |
| `mutable` member | `Cell` / `RefCell` |
| destructor | `Drop` |

- In GC languages every object is effectively `Rc<RefCell<T>>`. In Rust you opt in only where you need it.
- Reference cycles of `Rc` leak memory. Break them with `Weak`.
- If you reach for `Rc<RefCell<..>>` often, consider owning data in one place and passing indices or IDs around.

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
