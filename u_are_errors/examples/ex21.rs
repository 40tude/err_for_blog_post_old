// ex21.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex21
// cargo add serde --features derive
// cargo add serde_json
// cargo add anyhow

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::{read_to_string, write};
use std::io::{self, ErrorKind};

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

fn load_or_init(path: &str) -> Result<Config> {
    match load_config(path) {
        Ok(cfg) => Ok(cfg),

        Err(err) => {
            // Try to see if this error (or one of its sources) is an io::Error
            if let Some(ioe) = err.downcast_ref::<io::Error>() {
                if ioe.kind() == ErrorKind::NotFound {
                    // I/O: file missing → create a default and continue
                    let default = Config { app_name: "Demo".into(), port: 8086 };
                    let default_json = r#"{ "app_name": "Demo", "port": 8086 }"#;
                    write(path, default_json).with_context(|| format!("failed to write default config to {path}"))?;
                    eprintln!("{path} not found, created with default config");
                    return Ok(default);
                } else {
                    // I/O: other I/O errors → log and bubble up
                    eprintln!("I/O error accessing {path}: {ioe}");
                    return Err(err);
                }
            }

            // Not an io::Error; see if it's a serde_json::Error (parse failure)
            if let Some(parsee) = err.downcast_ref::<serde_json::Error>() {
                // Parse error: file exists but JSON is invalid → log and bubble up
                eprintln!("Invalid JSON in {path}: {parsee}");
                return Err(err);
            }

            // Fallback: unknown kind of error — just bubble it up.
            Err(err)
        }
    }
}
fn main() -> Result<()> {
    // Create demo files
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#).context("writing good_config.json")?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#).context("writing bad_config.json")?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
