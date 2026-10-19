// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 5-1 · Many Forms                                 125 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   An `enum` is a type that is exactly ONE of several variants:
//
//     enum Weather { Sunny, Rainy, Foggy }
//
//     let today = Weather::Rainy;            // `Type::Variant`
//
//     let mood = match today {
//         Weather::Sunny => "happy",
//         Weather::Rainy => "soggy",
//         Weather::Foggy => "lost",
//     };                                     // no `_` needed: all covered!
//
//   • `match` on an enum must cover EVERY variant. If someone adds a
//     variant later, the compiler shows every match that needs updating.
//     That's one of Rust's superpowers.
//   • Enums can't be compared with `==` unless you
//     `#[derive(PartialEq)]` (same as structs in lesson 4).
//
// 🎯 YOUR TASK
//
//   Travel costs: Swamp 3, Forest 2, Mountain 5, River 1.
//   Make the program print:
//
//       Total travel cost: 11
//       Squelch!

enum Terrain {
    Swamp,
    Forest,
    Mountain,
    River,
}

fn travel_cost(t: Terrain) -> u32 {
    match t {
        Terrain::Swamp => 3,
        Terrain::Forest => 2,
        Terrain::Mountain => 5,
    }
}

fn main() {
    let route = [Terrain::Forest, Terrain::Swamp, Terrain::River, Terrain::Mountain];
    let mut total = 0;
    for t in route {
        total += travel_cost(t);
    }
    println!("Total travel cost: {total}");

    let here = Terrain::Swamp;
    if here == Terrain::Swamp {
        println!("Squelch!");
    }
}
