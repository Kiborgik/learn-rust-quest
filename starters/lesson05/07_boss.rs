// ╔══════════════════════════════════════════════════════════════════╗
// ║  👹 BOSS · The Hydra of Variants                          300 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
//   In the heart of the swamp the Hydra waits, and every one of its
//   heads is a different variant. You command a hero through a tiny
//   command language. The TYPES are given; write the logic.
//
//   ⚔️  HEAD 1: `parse_dir(word: &str) -> Option<Dir>`
//        "n" or "north" → North, "s"/"south", "e"/"east", "w"/"west",
//        anything else → None.   (You can match on &str literals!)
//
//   ⚔️  HEAD 2: `Hero::new()` and the `Move` command
//        A new hero: x 0, y 0, hp 50, mana 10.
//        `hero.apply(&cmd) -> String` executes a command and returns a message.
//        Move(dir, steps): North → y += steps, South → y -= steps,
//        East → x += steps, West → x -= steps.  Message: "Moved to (x, y)"
//
//   ⚔️  HEAD 3: the other commands
//        Attack { target, power }: costs 2 mana.
//            mana < 2       → "Too tired to attack {target}" (no mana spent)
//            power >= 50    → "CRITICAL hit on {target} for {power}!"
//            otherwise      → "Hit {target} for {power}"
//        Heal(n): hp += n, max 100.            → "Healed to {hp}"
//        Rest: mana back to 10.                → "Rested"
//        Quit:                                 → "Farewell"
//
//   ⚔️  HEAD 4: `run(cmds: &[Command]) -> Hero`
//        A fresh hero applies the commands in order, STOPPING at Quit.
//
//   ⚔️  HEAD 5: `is_movement(cmd: &Command) -> bool` using `matches!`
//
//   💎 BONUS: `last_target(cmds: &[Command]) -> Option<&str>`
//        The target of the LAST Attack command, if any.
//        (`.as_str()` turns a `&String` into a `&str`.)
//        (`for c in cmds.iter().rev()` loops backwards.)

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq)]
enum Command {
    Move(Dir, i32),
    Attack { target: String, power: u32 },
    Heal(u32),
    Rest,
    Quit,
}

#[derive(Debug, PartialEq)]
struct Hero {
    x: i32,
    y: i32,
    hp: u32,
    mana: u32,
}

// Your code goes here ↓



fn main() {
    println!("Seven heads hiss at you. Run the tests to fight the Hydra!");
}

// ─── The Hydra's tests: don't change these! ──────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn attack(target: &str, power: u32) -> Command {
        Command::Attack { target: target.to_string(), power }
    }

    #[test]
    fn head_1_directions() {
        assert_eq!(parse_dir("n"), Some(Dir::North));
        assert_eq!(parse_dir("south"), Some(Dir::South));
        assert_eq!(parse_dir("e"), Some(Dir::East));
        assert_eq!(parse_dir("west"), Some(Dir::West));
        assert_eq!(parse_dir("up"), None);
    }

    #[test]
    fn head_2_moving() {
        let mut h = Hero::new();
        assert_eq!(h, Hero { x: 0, y: 0, hp: 50, mana: 10 });
        assert_eq!(h.apply(&Command::Move(Dir::East, 3)), "Moved to (3, 0)");
        assert_eq!(h.apply(&Command::Move(Dir::North, 2)), "Moved to (3, 2)");
        assert_eq!(h.apply(&Command::Move(Dir::West, 5)), "Moved to (-2, 2)");
        assert_eq!(h.apply(&Command::Move(Dir::South, 1)), "Moved to (-2, 1)");
    }

    #[test]
    fn head_3_fighting() {
        let mut h = Hero::new();
        assert_eq!(h.apply(&attack("Slime", 10)), "Hit Slime for 10");
        assert_eq!(h.mana, 8);
        assert_eq!(h.apply(&attack("Hydra", 60)), "CRITICAL hit on Hydra for 60!");
        h.apply(&attack("Slime", 1));
        h.apply(&attack("Slime", 1));
        h.apply(&attack("Slime", 1));
        assert_eq!(h.mana, 0);
        assert_eq!(h.apply(&attack("Slime", 1)), "Too tired to attack Slime");
        assert_eq!(h.mana, 0);
        assert_eq!(h.apply(&Command::Rest), "Rested");
        assert_eq!(h.mana, 10);
        assert_eq!(h.apply(&Command::Heal(70)), "Healed to 100");
        assert_eq!(h.apply(&Command::Quit), "Farewell");
    }

    #[test]
    fn head_4_running() {
        let cmds = [
            Command::Move(Dir::North, 1),
            Command::Heal(5),
            Command::Quit,
            Command::Move(Dir::North, 100),
        ];
        let h = run(&cmds);
        assert_eq!(h.y, 1);
        assert_eq!(h.hp, 55);
    }

    #[test]
    fn head_5_movement() {
        assert!(is_movement(&Command::Move(Dir::East, 1)));
        assert!(!is_movement(&Command::Rest));
        assert!(!is_movement(&attack("x", 1)));
    }

    #[test]
    fn bonus_last_target() {
        let cmds = [attack("Slime", 1), Command::Rest, attack("Hydra", 2), Command::Heal(1)];
        assert_eq!(last_target(&cmds), Some("Hydra"));
        assert_eq!(last_target(&[Command::Rest]), None);
    }
}
