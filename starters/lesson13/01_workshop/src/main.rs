// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 13-1 · Open the Workshop                         150 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   From now on every quest is a real Cargo project (a "package"):
//
//     01_workshop/
//     ├── Cargo.toml        name, version, edition, dependencies
//     └── src/
//         ├── main.rs       the binary's entry point (this file)
//         └── tools.rs      a module
//
//   `cargo new my_app` creates exactly this layout. `cargo run` builds and
//   runs it, `cargo test` runs its tests.
//
//   MODULES: a file is NOT compiled just because it exists. You declare it:
//
//     mod tools;            // "load src/tools.rs as the module `tools`"
//     tools::count();       // call into it with `module::item`
//
//   Cargo.toml values are available at compile time:
//
//     env!("CARGO_PKG_NAME")       // "l13_01_workshop"
//     env!("CARGO_PKG_VERSION")    // "0.1.0"
//
//   Coming from other languages: there's no classpath or import scanning.
//   `mod` builds the module tree, starting from main.rs (or lib.rs).
//
// 🎯 YOUR TASK
//
//   1. Make `tools.rs` part of the program.
//   2. The workshop is ready for release: set the version in Cargo.toml
//      to 1.0.0.
//
//   Expected output:
//
//       Ferris's Workshop v1.0.0
//       Tools on the wall: 3

fn main() {
    println!("{}", tools::banner());
    println!("Tools on the wall: {}", tools::count());
}
