// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 30-2 · Doors That Know                           175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   TYPESTATE puts an object's state in its TYPE, so wrong moves don't
//   compile. A door is Locked, Unlocked or Open, and each state has only
//   the methods that make sense:
//
//     pub struct Door<S> { code: u32, _state: PhantomData<S> }
//
//     impl Door<Locked> {
//         pub fn unlock(self, code: u32) -> Result<Door<Unlocked>, Door<Locked>> { ... }
//     }
//     impl Door<Unlocked> {
//         pub fn open(self) -> Door<Open> { ... }
//     }
//
//     let door = Door::new(1234);
//     door.open();        // ❌ compile error: no `open` on Door<Locked>
//
//   Methods take `self` by value, so the old state is gone after the
//   move: you can't keep using a door you've just unlocked as if it were
//   still locked.
//
//   When a move can fail, hand the original back in the error:
//   `Err(self)`. Then the caller still has the locked door and can try
//   again. Never lose the caller's value.
//
//   (Real uses: connection builders, HTTP request builders that need a
//   URL before `send`, file handles that must be opened before reading.)
//
// 🎯 YOUR TASK
//
//   1. A wrong code destroys the door's secret. Return the same door.
//   2. Write the Unlocked and Open methods (see the tests for what each
//      state can do).

use std::marker::PhantomData;

pub struct Locked;
pub struct Unlocked;
pub struct Open;

pub struct Door<S> {
    code: u32,
    _state: PhantomData<S>,
}

impl<S> Door<S> {
    fn into_state<T>(self) -> Door<T> {
        Door { code: self.code, _state: PhantomData }
    }
}

impl Door<Locked> {
    pub fn new(code: u32) -> Door<Locked> {
        Door { code, _state: PhantomData }
    }

    pub fn unlock(self, code: u32) -> Result<Door<Unlocked>, Door<Locked>> {
        if code == self.code { Ok(self.into_state()) } else { Err(Door::new(0)) }
    }
}

impl Door<Unlocked> {
    // pub fn open(self) -> Door<Open>
    // pub fn lock(self) -> Door<Locked>
}

impl Door<Open> {
    // pub fn enter(&self, hero: &str) -> String     "Ferris walks in"
    // pub fn close(self) -> Door<Unlocked>
}

fn main() {
    let door = Door::new(1234);
    // door.open();   // try it: this line doesn't compile
    let _ = door.unlock(1234);
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walking_through() {
        let door = Door::new(1234).unlock(1234).ok().unwrap();
        let open = door.open();
        assert_eq!(open.enter("Ferris"), "Ferris walks in");
        assert_eq!(open.enter("Lumi"), "Lumi walks in");
        let locked: Door<Locked> = open.close().lock();
        assert!(locked.unlock(1234).is_ok());
    }

    #[test]
    fn wrong_code_keeps_the_door() {
        let door = Door::new(1234);
        let door = match door.unlock(1111) {
            Ok(_) => panic!("wrong code opened the door"),
            Err(still_locked) => still_locked,
        };
        assert!(door.unlock(1234).is_ok(), "the right code must still work");
    }
}
