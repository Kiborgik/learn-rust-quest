// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 21-3 · All Hands                                 175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   Awaiting one after another waits for each in turn:
//
//     let a = forge().await;     // 2 s
//     let b = brew().await;      // then 1 s more   → 3 s total
//
//   `join!` runs futures concurrently on the same task and waits for all
//   of them (no spawning, borrowing allowed):
//
//     let (a, b) = tokio::join!(forge(), brew());     // 2 s total
//
//   `try_join!` does the same for futures that return Result, and stops
//   at the first error:
//
//     let (a, b) = tokio::try_join!(forge_checked(), brew_checked())?;
//
//   join! vs spawn: join! keeps everything in the current task (cheap,
//   can borrow). spawn creates independent tasks that can run in
//   parallel on other threads.
//
// 🎯 YOUR TASK
//
//   1. `prepare` runs its steps one by one. Make them concurrent.
//   2. Implement `gear_up` with `try_join!`.

use std::time::Duration;
use tokio::time::sleep;

async fn forge() -> String {
    sleep(Duration::from_secs(2)).await;
    "sword".into()
}

async fn brew() -> String {
    sleep(Duration::from_secs(1)).await;
    "potion".into()
}

async fn count_gold() -> u32 {
    sleep(Duration::from_secs(1)).await;
    40
}

pub async fn prepare() -> (String, String, u32) {
    let sword = forge().await;
    let potion = brew().await;
    let gold = count_gold().await;
    (sword, potion, gold)
}

async fn check_armor(ok: bool) -> Result<u32, String> {
    sleep(Duration::from_secs(1)).await;
    if ok { Ok(10) } else { Err("armor is cracked".into()) }
}

async fn check_boots(ok: bool) -> Result<u32, String> {
    sleep(Duration::from_secs(3)).await;
    if ok { Ok(5) } else { Err("boots have holes".into()) }
}

/// Both checks at once. Ok(armor + boots) or the first error.
pub async fn gear_up(armor_ok: bool, boots_ok: bool) -> Result<u32, String> {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn preparing_is_concurrent() {
        let start = Instant::now();
        assert_eq!(prepare().await, ("sword".into(), "potion".into(), 40));
        assert_eq!(start.elapsed().as_secs(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn gearing_up() {
        let start = Instant::now();
        assert_eq!(gear_up(true, true).await, Ok(15));
        assert_eq!(start.elapsed().as_secs(), 3);
        assert_eq!(gear_up(false, true).await, Err("armor is cracked".to_string()));
    }
}
