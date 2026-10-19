// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 6-2 · Inventory Check                            125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Three ways to loop over a Vec, following the ownership rules again:
//
//     let mut v = vec![1, 2, 3];
//
//     for x in &v { }          // x: &i32      READ (v still usable)
//     for x in &mut v {        // x: &mut i32  MODIFY in place
//         *x *= 10;            // `*` to change the value behind the ref
//     }
//     for x in v { }           // x: i32       CONSUME (v is moved!)
//
//   A classic error that Rust catches at compile time:
//
//     let first = &v[0];       // borrow an element...
//     v.push(4);               // ✘ ERROR: push may REALLOCATE and leave
//     println!("{first}");     //   `first` pointing at freed memory
//
//   In C++ this is undefined behaviour. In Java it's a
//   ConcurrentModificationException at runtime. In Rust it doesn't
//   compile.
//
// 🎯 YOUR TASK
//
//   Three of these four functions don't compile. Fix them so all the
//   tests pass. (Why does `sum += w` work in `total`, even though `w` is
//   a `&u32`? Integers implement `+=` for references too. Handy!)

/// Total weight of all items.
fn total(weights: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for w in weights {
        sum += w;
    }
    sum
}

/// The blacksmith repairs everything: +10 durability each.
fn repair_all(durability: &mut Vec<u32>) {
    for d in durability {
        d += 10;
    }
}

/// Add a feather (weight 1), then a copy of the first item's weight.
fn duplicate_first(weights: &mut Vec<u32>) {
    let first = &weights[0];
    weights.push(1);
    weights.push(*first);
}

/// Turn names into shouted names, consuming the original list.
fn shout_all(names: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for n in &names {
        out.push(n + "!");
    }
    out
}

fn main() {
    let mut v = vec![3, 5, 7];
    repair_all(&mut v);
    duplicate_first(&mut v);
    println!("{:?} total {}", v, total(&v));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals() {
        assert_eq!(total(&vec![1, 2, 3]), 6);
    }

    #[test]
    fn repairs() {
        let mut v = vec![0, 5, 90];
        repair_all(&mut v);
        assert_eq!(v, vec![10, 15, 100]);
    }

    #[test]
    fn duplicates() {
        let mut v = vec![4, 2];
        duplicate_first(&mut v);
        assert_eq!(v, vec![4, 2, 1, 4]);
    }

    #[test]
    fn shouting() {
        let names = vec![String::from("hi"), String::from("yo")];
        assert_eq!(shout_all(names), vec!["hi!", "yo!"]);
    }
}
