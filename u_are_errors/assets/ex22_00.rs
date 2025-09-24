// ex22.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex22
// cargo add serde --features derive
// cargo add serde_json

mod my_api {
    use serde::Deserialize;
    use std::error::Error;
    use std::fmt;
    use std::fs::read_to_string;

    type Result<T> = std::result::Result<T, ConfigError>;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub app_name: String,
        pub port: u16,
    }

    #[derive(Debug)]
    pub enum ConfigError {
        Io(std::io::Error),
        Parse(serde_json::Error),
    }

    // Implement Display for our error to satisfy Error trait.
    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ConfigError::Io(e) => write!(f, "I/O error: {e}"),
                ConfigError::Parse(e) => write!(f, "Parse error: {e}"),
            }
        }
    }

    // Implement the standard Error trait for integration with other error tooling.
    impl Error for ConfigError {}

    // Map the inner errors explicitly (uses map_err).
    pub fn load_config(path: &str) -> Result<Config> {
        let data = read_to_string(path).map_err(ConfigError::Io)?;
        let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
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
