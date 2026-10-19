// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Forbidden Archive                         400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   Deep in the archive lies a stack that never touches the heap: a
//   fixed-size array that holds up to N items. Its slots start EMPTY, so
//   it can't be a plain `[T; N]` (that needs N real values up front).
//
//   `MaybeUninit<T>` is a slot that may or may not hold a T. Rust won't
//   read it or drop it for you. YOU track which slots are filled (`len`).
//
//     let mut slot = MaybeUninit::<String>::uninit();
//     slot.write("hi".to_string());                   // safe: fill it
//     let s = unsafe { slot.assume_init_read() };     // move it out
//     let r = unsafe { slot.assume_init_ref() };      // borrow it
//     unsafe { slot.assume_init_drop() };             // drop it in place
//
//   Each `assume_init_*` is a promise that the slot is filled. Reading an
//   empty slot, or reading a value twice, is undefined behavior.
//
//   Empty array of slots: `[const { MaybeUninit::uninit() }; N]`.
//
//   ⚔️  HEAD 1: `new`, `len`, `push` (Err(value) back when full),
//        `pop`, `peek`.
//   ⚔️  HEAD 2: `as_slice`: the filled part as &[T]. The filled slots sit
//        side by side, so `slice::from_raw_parts` can view them (cast the
//        array pointer from *const MaybeUninit<T> to *const T).
//   ⚔️  HEAD 3: `Drop`: drop every item still on the stack, exactly once.
//        Nothing leaks, nothing is dropped twice.
//   ⚔️  HEAD 4: every unsafe block has a `// SAFETY:` comment. (The
//        tests can't check this one. Your future self can.)

use std::mem::MaybeUninit;

pub struct ArrayStack<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> ArrayStack<T, N> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn as_slice(&self) -> &[T] {
        todo!()
    }
}

fn main() {
    let mut s: ArrayStack<String, 4> = ArrayStack::new();
    s.push("scroll".into()).unwrap();
    println!("{:?}", s.as_slice());
}

// ─── The Archive's tests: don't change these! ────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// Counts how many times it was dropped.
    #[derive(Debug)]
    struct Relic(u32, Rc<Cell<u32>>);

    impl Drop for Relic {
        fn drop(&mut self) {
            self.1.set(self.1.get() + 1);
        }
    }

    #[test]
    fn head_1_push_pop_peek() {
        let mut s: ArrayStack<String, 2> = ArrayStack::new();
        assert_eq!(s.len(), 0);
        assert_eq!(s.pop(), None);
        assert_eq!(s.peek(), None);
        s.push("a".into()).unwrap();
        s.push("b".into()).unwrap();
        assert_eq!(s.push("c".into()), Err("c".to_string()), "full: the value comes back");
        assert_eq!(s.peek().map(String::as_str), Some("b"));
        assert_eq!(s.pop().as_deref(), Some("b"));
        assert_eq!(s.pop().as_deref(), Some("a"));
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn head_2_slice() {
        let mut s: ArrayStack<u32, 8> = ArrayStack::new();
        assert!(s.as_slice().is_empty());
        for x in [3, 1, 4, 1, 5] {
            s.push(x).unwrap();
        }
        assert_eq!(s.as_slice(), [3, 1, 4, 1, 5]);
        s.pop();
        assert_eq!(s.as_slice().iter().sum::<u32>(), 9);
    }

    #[test]
    fn head_3_drops() {
        let drops = Rc::new(Cell::new(0));
        {
            let mut s: ArrayStack<Relic, 4> = ArrayStack::new();
            for i in 0..3 {
                s.push(Relic(i, drops.clone())).unwrap();
            }
            let top = s.pop().unwrap();
            assert_eq!(top.0, 2);
            drop(top);
            assert_eq!(drops.get(), 1);
            s.push(Relic(7, drops.clone())).unwrap();
            s.push(Relic(8, drops.clone())).unwrap();
            let refused = s.push(Relic(9, drops.clone())).unwrap_err();
            drop(refused);
            assert_eq!(drops.get(), 2);
        }
        assert_eq!(drops.get(), 6, "every relic dropped exactly once");
    }
}
