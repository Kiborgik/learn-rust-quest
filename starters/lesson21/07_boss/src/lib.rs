// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Courier Guild                             400 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   The Courier Guild delivers parcels across the realm, and its master
//   wants everything faster, but never more couriers on the road than
//   the guild has. Build the dispatch system.
//
//   `deliver` is given: it takes `distance` seconds, and fragile parcels
//   break on long trips.
//
//   ⚔️  HEAD 1: `deliver_all(parcels, couriers) -> Vec<Result<..>>`
//        Deliver every parcel, at most `couriers` at the same time.
//        Results in the order of `parcels`.
//        Tip: a `Semaphore` hands out a fixed number of permits. Put it
//        in an Arc, give each spawned task a clone, and take a permit
//        before delivering. A task without one waits in `acquire`:
//
//            let sem = Arc::new(Semaphore::new(2));    // 2 permits
//            let sem2 = sem.clone();                   // for one task
//            let permit = sem2.acquire_owned().await.unwrap();
//            // ... at most 2 tasks are here at the same time ...
//            drop(permit);         // or let it drop at the end
//
//        Keep the JoinHandles in order and await them in order.
//
//   ⚔️  HEAD 2: timing. 6 parcels of distance 1 with 3 couriers take 2
//        seconds; with 1 courier, 6.
//
//   ⚔️  HEAD 3: `deliver_with_timeout(p, max_secs)`:
//        Err("parcel {id} timed out") if it takes longer than max_secs.
//
//   ⚔️  HEAD 4: `fastest(parcels) -> Option<u32>`: deliver all at once
//        and return the id of the first one to finish (the others are
//        cancelled). Tip: a `JoinSet` holds many spawned tasks:
//
//            let mut set = JoinSet::new();
//            set.spawn(async move { 7 });       // like tokio::spawn
//            set.join_next().await   // Some(Ok(7)): first to finish;
//                                    // None if the set is empty
//
//        Dropping the set cancels the tasks that are still running.
//
//   ⚔️  HEAD 5: `summary(results) -> String`:
//        "3 delivered, 1 failed" (all tests use plural forms)

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};

#[derive(Debug, Clone)]
pub struct Parcel {
    pub id: u32,
    pub distance: u64,
    pub fragile: bool,
}

pub async fn deliver(p: Parcel) -> Result<String, String> {
    sleep(Duration::from_secs(p.distance)).await;
    if p.fragile && p.distance > 5 {
        Err(format!("parcel {} broke", p.id))
    } else {
        Ok(format!("parcel {} delivered", p.id))
    }
}

pub async fn deliver_all(parcels: Vec<Parcel>, couriers: usize) -> Vec<Result<String, String>> {
    todo!()
}

pub async fn deliver_with_timeout(p: Parcel, max_secs: u64) -> Result<String, String> {
    todo!()
}

pub async fn fastest(parcels: Vec<Parcel>) -> Option<u32> {
    todo!()
}

pub fn summary(results: &[Result<String, String>]) -> String {
    todo!()
}

// ─── The Guild's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    fn p(id: u32, distance: u64, fragile: bool) -> Parcel {
        Parcel { id, distance, fragile }
    }

    #[tokio::test(start_paused = true)]
    async fn head_1_order() {
        let r = deliver_all(vec![p(1, 3, false), p(2, 1, false), p(3, 7, true)], 2).await;
        assert_eq!(
            r,
            vec![
                Ok("parcel 1 delivered".to_string()),
                Ok("parcel 2 delivered".to_string()),
                Err("parcel 3 broke".to_string()),
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn head_2_couriers_limit() {
        let parcels: Vec<Parcel> = (1..=6).map(|i| p(i, 1, false)).collect();
        let start = Instant::now();
        deliver_all(parcels.clone(), 3).await;
        assert_eq!(start.elapsed().as_secs(), 2);
        let start = Instant::now();
        deliver_all(parcels, 1).await;
        assert_eq!(start.elapsed().as_secs(), 6);
    }

    #[tokio::test(start_paused = true)]
    async fn head_3_timeout() {
        assert_eq!(deliver_with_timeout(p(9, 2, false), 5).await, Ok("parcel 9 delivered".into()));
        assert_eq!(deliver_with_timeout(p(9, 20, false), 5).await, Err("parcel 9 timed out".into()));
    }

    #[tokio::test(start_paused = true)]
    async fn head_4_fastest() {
        let start = Instant::now();
        assert_eq!(fastest(vec![p(1, 9, false), p(2, 4, false), p(3, 6, false)]).await, Some(2));
        assert_eq!(start.elapsed().as_secs(), 4);
        assert_eq!(fastest(vec![]).await, None);
    }

    #[test]
    fn head_5_summary() {
        let r = vec![Ok("a".into()), Err("b".into()), Ok("c".into()), Ok("d".into())];
        assert_eq!(summary(&r), "3 delivered, 1 failed");
        assert_eq!(summary(&[]), "0 delivered, 0 failed");
    }
}
