// ex23.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex23
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

use my_api::{Config, ConfigError, load_config};
use std::fs::write;
use std::io::ErrorKind;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn load_or_init(path: &str) -> Result<Config> {
    match load_config(path) {
        Ok(cfg) => Ok(cfg),

        // I/O: file missing → create a default one and continue
        Err(ConfigError::Io(ref e)) if e.kind() == ErrorKind::NotFound => {
            let default = Config { app_name: "Demo".into(), port: 8080 };
            // Keep it simple: write a literal rather than serializing
            write(path, r#"{ "app_name": "Demo", "port": 8080 }"#)?;
            Ok(default)
        }

        // I/O: other I/O errors → report and bubble up
        Err(ConfigError::Io(e)) => {
            eprintln!("I/O error accessing {path}: {e}");
            Err(e.into())
        }

        // Parse error: the file exists but JSON is invalid → give a helpful message
        Err(ConfigError::Parse(e)) => {
            eprintln!("Invalid JSON in {path}: {e}");
            Err(e.into())
        }
    }
}

fn main() -> Result<()> {
    let cfg = load_or_init("config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
