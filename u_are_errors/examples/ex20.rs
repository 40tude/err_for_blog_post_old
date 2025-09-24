// ex20.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex20
// cargo add anyhow

use anyhow::{Context, Result};
use std::fs;

// Result alias = Result<T, anyhow::Error>
fn run() -> Result<()> {
    let data = fs::read_to_string("config.json").context("While reading config.json")?; // adds context if it fails
    let cfg: serde_json::Value = serde_json::from_str(&data).context("While parsing JSON")?;
    println!("Config loaded: {cfg}");
    Ok(())
}

fn main() -> Result<()> {
    run()
}
