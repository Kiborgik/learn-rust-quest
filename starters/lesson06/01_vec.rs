// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 6-1 · The Backpack                               125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   `Vec<T>` is a growable array on the heap (like ArrayList / List<T>
//   / a JS array, but holding ONE type):
//
//     let mut bag: Vec<&str> = Vec::new();   // empty: needs a type hint
//     let mut nums = vec![1, 2, 3];          // vec! macro: type inferred
//
//     bag.push("rope");                       // add to the end
//     let last = nums.pop();                  // Option<i32>: Some(3)
//     nums.len();                             // 2
//
//     let first = nums[0];                    // index: PANICS if out of range
//     let maybe = nums.get(10);               // Option<&i32>: None, no panic
//
//     nums.insert(0, 99);                     // insert at index
//     nums.remove(1);                         // remove at index, returns it
//     nums.contains(&99);                     // true
//     nums.is_empty();                        // false
//
//   `{:?}` prints a whole Vec: [99, 2]
//
// 🎯 YOUR TASK
//
//   Make the program print:
//
//       Backpack: ["rope", "torch", "map"]
//       Dropped: map
//       Slot 5: empty
//       Items left: 2

fn main() {
    let backpack = Vec::new();
    backpack.push("rope");
    backpack.push("torch");
    backpack.push("map");
    println!("Backpack: {:?}", backpack);

    let dropped = backpack.pop();
    println!("Dropped: {dropped}");

    let slot5 = match backpack[5] {
        Some(item) => item,
        None => "empty",
    };
    println!("Slot 5: {slot5}");

    println!("Items left: {}", backpack.len());
}
