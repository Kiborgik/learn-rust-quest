// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 12-4 · The Royal Seal                            150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A `Mutex<T>` (mutual exclusion) protects data so that only one
//   thread at a time can access it. In Rust the lock OWNS the data,
//   so you can't touch the data without holding the lock:
//
//     use std::sync::Mutex;
//
//     let honey = Mutex::new(0);
//     {
//         let mut guard = honey.lock().unwrap();   // MutexGuard<i32>
//         *guard += 5;                             // deref to the data
//     }                                            // guard dropped → unlocked
//
//     *honey.lock().unwrap() += 1;     // one-liner: lock, change, unlock
//
//   • `lock()` returns a Result: Err if a thread panicked while holding
//     the lock ("poisoned"). `unwrap()` is the usual answer.
//   • The lock is released when the GUARD is dropped. Holding a guard
//     and calling `lock()` again on the same thread DEADLOCKS. Keep
//     guards short-lived!
//   • `Mutex::lock` takes `&self`: it's interior mutability, like
//     RefCell, but thread-safe.
//
// 🎯 YOUR TASK (single-threaded for now)
//
//   1. Fix `deposit` (it can't modify the honey without locking).
//   2. Implement `withdraw`: take `amount` if there is enough; return
//      whether it worked.
//   3. Implement `royal_tax`: take 10% (integer division) of the jar and
//      return how much was taken. Lock only ONCE in this function.

use std::sync::Mutex;

struct Jar {
    honey: Mutex<u32>,
}

impl Jar {
    fn new(honey: u32) -> Self {
        Jar { honey: Mutex::new(honey) }
    }

    fn deposit(&self, amount: u32) {
        self.honey += amount;
    }

    fn withdraw(&self, amount: u32) -> bool {
        todo!()
    }

    fn royal_tax(&self) -> u32 {
        todo!()
    }

    fn amount(&self) -> u32 {
        *self.honey.lock().unwrap()
    }
}

fn main() {
    let jar = Jar::new(100);
    jar.deposit(50);
    println!("{}", jar.amount());
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depositing() {
        let jar = Jar::new(100);
        jar.deposit(50);
        assert_eq!(jar.amount(), 150);
    }

    #[test]
    fn withdrawing() {
        let jar = Jar::new(100);
        assert!(jar.withdraw(30));
        assert!(!jar.withdraw(500));
        assert_eq!(jar.amount(), 70);
    }

    #[test]
    fn taxes() {
        let jar = Jar::new(155);
        assert_eq!(jar.royal_tax(), 15);
        assert_eq!(jar.amount(), 140);
    }
}
