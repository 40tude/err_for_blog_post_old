// ex24.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex24
// cargo add serde --features derive
// cargo add serde_json
// cargo add thiserror

mod my_api {
    use serde::Deserialize;
    use std::fs::read_to_string;
    use thiserror::Error;

    type Result<T> = std::result::Result<T, ConfigError>;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub app_name: String,
        pub port: u16,
    }

    /// Public, well-typed error exposed by the library.
    #[derive(Debug, Error)]
    pub enum ConfigError {
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        #[error("JSON parse error: {0}")]
        Json(#[from] serde_json::Error),
    }

    // Thanks to `#[from]`, both I/O and JSON errors automatically convert into `ConfigError`—so `?` just works.
    pub fn load_config(path: &str) -> Result<Config> {
        let data = read_to_string(path)?; // -> ConfigError::Io
        let cfg = serde_json::from_str::<Config>(&data)?; // -> ConfigError::Json
        Ok(cfg)
    }
}

use my_api::load_config;
use std::fs::write;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Create demo files
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

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
