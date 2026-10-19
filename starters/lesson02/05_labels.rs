// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 2-5 · The Nested Mines                           100 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `break` and `continue` affect the INNERMOST loop. To control an
//   outer loop, give it a LABEL (an apostrophe + a name):
//
//     'search: for x in 0..10 {
//         for y in 0..10 {
//             if x * y == 42 {
//                 println!("found {x} * {y}");
//                 break 'search;          // leaves BOTH loops
//             }
//             if y > x {
//                 continue 'search;       // jumps to the next `x`
//             }
//         }
//     }
//
//   2D arrays are arrays of arrays: `[[u32; 4]; 3]` = 3 rows of 4 numbers.
//
//     let grid = [[1, 2], [3, 4]];
//     for row in grid {
//         for cell in row { /* ... */ }
//     }
//
// 🎯 YOUR TASK
//
//   1. `gems_before_cave_in`: dig row by row, cell by cell, collecting
//      gems. A 0 means a CAVE-IN: stop digging the whole mine at once.
//      Right now it only stops the current row. Fix it with a label.
//   2. `safe_rows`: count rows that contain NO cursed gem (13).
//      Use `continue 'label` to skip to the next row as soon as you
//      spot a 13.

fn gems_before_cave_in(mine: [[u32; 4]; 3]) -> u32 {
    let mut gems = 0;
    for row in mine {
        for cell in row {
            if cell == 0 {
                break; // a cave-in! we should stop digging ENTIRELY
            }
            gems += cell;
        }
    }
    gems
}

fn safe_rows(mine: [[u32; 4]; 3]) -> u32 {
    todo!()
}

fn main() {
    let mine = [[1, 2, 3, 4], [5, 0, 7, 8], [9, 9, 9, 9]];
    println!("Gems collected: {}", gems_before_cave_in(mine));

    let cursed = [[1, 13, 1, 1], [2, 2, 2, 2], [3, 3, 3, 13]];
    println!("Safe rows: {}", safe_rows(cursed));
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cave_in_stops_everything() {
        let mine = [[1, 2, 3, 4], [5, 0, 7, 8], [9, 9, 9, 9]];
        assert_eq!(gems_before_cave_in(mine), 15);
    }

    #[test]
    fn no_cave_in() {
        let mine = [[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]];
        assert_eq!(gems_before_cave_in(mine), 12);
    }

    #[test]
    fn cursed_rows() {
        let mine = [[1, 13, 1, 1], [2, 2, 2, 2], [3, 3, 3, 13]];
        assert_eq!(safe_rows(mine), 1);
        let clean = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        assert_eq!(safe_rows(clean), 3);
    }
}
