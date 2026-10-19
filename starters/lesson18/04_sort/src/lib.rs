// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 18-4 · The Great Sorting                         175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   rayon's ParallelSliceMut trait adds parallel sorts to every slice:
//
//     v.par_sort();                            // stable, like sort()
//     v.par_sort_unstable();                   // faster, equal items may swap
//     v.par_sort_by_key(|h| h.level);
//     v.par_sort_by(|a, b| b.score.cmp(&a.score));   // highest first
//
//   They're merge sort / quicksort split across threads. Worth it for big
//   slices (tens of thousands of items and up).
//
//   Stable vs unstable: a stable sort keeps equal elements in their
//   original order. That matters when you sort by one key after another.
//
//   `Ordering::then` combines comparisons:
//
//     v.par_sort_by(|a, b| b.level.cmp(&a.level).then(a.name.cmp(&b.name)));
//
// 🎯 YOUR TASK
//
//   Implement `rank`: sort heroes by level, highest first; equal levels by
//   name A→Z. Use a rayon sort.

use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Hero {
    pub name: String,
    pub level: u32,
}

pub fn rank(heroes: &mut [Hero]) {
    todo!()
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn h(name: &str, level: u32) -> Hero {
        Hero { name: name.into(), level }
    }

    #[test]
    fn small() {
        let mut v = vec![h("Lumi", 3), h("Ferris", 9), h("Corro", 9), h("Ada", 1)];
        rank(&mut v);
        let names: Vec<&str> = v.iter().map(|h| h.name.as_str()).collect();
        assert_eq!(names, vec!["Corro", "Ferris", "Lumi", "Ada"]);
    }

    #[test]
    fn large() {
        let mut v: Vec<Hero> = (0..50_000u32).map(|i| h(&format!("h{i:05}"), i % 100)).collect();
        rank(&mut v);
        assert_eq!(v[0], h("h00099", 99));
        assert_eq!(v[1], h("h00199", 99));
        assert_eq!(v.last().unwrap(), &h("h49900", 0));
    }
}
