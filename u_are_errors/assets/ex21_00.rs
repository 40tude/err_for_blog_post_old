// ex21.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex21
// cargo add serde --features derive
// cargo add serde_json
// cargo add anyhow

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::{read_to_string, write};

#[derive(Debug, Deserialize)]
struct Config {
    app_name: String,
    port: u16,
}

fn load_config(path: &str) -> Result<Config> {
    let data = read_to_string(path).with_context(|| format!("failed to read config file: {path}"))?;
    let cfg = serde_json::from_str::<Config>(&data).with_context(|| format!("failed to parse JSON in: {path}"))?;
    Ok(cfg)
}

fn main() -> Result<()> {
    // Create demo files
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#).context("writing good_config.json")?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#).context("writing bad_config.json")?;

    // Happy path
    println!("-- Loading good_config.json");
    match load_config("good_config.json") {
        Ok(cfg) => println!("OK  -> app_name='{}', port={}", cfg.app_name, cfg.port),
        Err(e) => println!("ERR -> {e} (debug: {e:?})"),
    }

    // JSON parse failure
    println!("\n-- Loading bad_config.json (should parse-fail)");
    match load_config("bad_config.json") {
        Ok(cfg) => println!("OK  -> app_name='{}', port={}", cfg.app_name, cfg.port),
        Err(e) => println!("ERR -> {e} (debug: {e:?})"),
    }

    // I/O failure (file does not exist)
    println!("\n-- Loading missing.json (should I/O-fail)");
    match load_config("missing.json") {
        Ok(cfg) => println!("OK  -> app_name='{}', port={}", cfg.app_name, cfg.port),
        Err(e) => println!("ERR -> {e} (debug: {e:?})"),
    }

    Ok(())
}
