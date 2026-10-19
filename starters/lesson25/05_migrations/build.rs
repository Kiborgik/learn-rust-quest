// Rebuild when a migration is added or changed. `sqlx::migrate!` copies
// the files into the program at compile time, and cargo can't know about
// them without this.
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
