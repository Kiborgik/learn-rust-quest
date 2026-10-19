//! The tool wall. This file is the module `tools`.

const NAME: &str = "Ferris's Workshop";

pub fn banner() -> String {
    format!("{NAME} v{}", env!("CARGO_PKG_VERSION"))
}

pub fn count() -> u32 {
    let tools = ["hammer", "tongs", "file"];
    tools.len() as u32
}
