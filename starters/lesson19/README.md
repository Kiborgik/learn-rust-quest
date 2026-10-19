# Lesson 19 · The Hall of Forms 🏛️

> Associated types and consts, operator traits, supertraits, blanket
> impls, the orphan rule, newtypes, PhantomData.

You'll meet all of these when reading library signatures (tokio, axum,
serde...). This lesson makes them readable.

## Associated items

```rust
trait Monster {
    type Loot;                 // associated type
    const DANGER: u32;         // associated const
    fn drop_loot(&self) -> Self::Loot;
}
fn f<M: Monster<Loot = u32>>(m: &M) -> u32 { M::DANGER + m.drop_loot() }
```

## Operator traits

```rust
impl Add<u32> for Gold {
    type Output = Gold;
    fn add(self, r: u32) -> Gold { ... }
}
impl AddAssign<u32> for Gold {
    fn add_assign(&mut self, r: u32) { ... }
}
```

`Add<Rhs = Self>`: the right-hand side defaults to the same type.

## Supertraits

```rust
// implementers must be both
trait Champion: Named + fmt::Display { ... }
Named::name(&x);  <Knight as Named>::name(&x); // disambiguation
```

## Blanket impls

```rust
impl<T: fmt::Display> Describe for T { ... }    // every Display type
```

A blanket impl and a specific impl for the same type conflict (E0119).

## Orphan rule + newtype

You can implement a trait for a type only if one of them is yours.
Wrap foreign types: `struct Inventory(Vec<Item>);`, then add `Deref` to
keep the inner methods.

## PhantomData

```rust
struct Money<C> { amount: u32, currency: PhantomData<C> }
impl Money<Gold> { fn to_silver(self) -> Money<Silver> { ... } }
```

Zero-size type markers: units, IDs, typestate.

## Coming from other languages

| Elsewhere | Rust |
|---|---|
| interface extends interface | supertrait |
| extension methods (C#, Kotlin) | blanket impls / local traits |
| C++ templates with tag types | PhantomData markers |
| operator overloading | `std::ops` traits |

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
