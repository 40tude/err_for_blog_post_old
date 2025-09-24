// ex24.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex24
// cargo add serde --features derive
// cargo add serde_json
// cargo add thiserror

mod my_api {
    use serde::Deserialize;
    use std::fs::{read_to_string, write};
    use std::io::ErrorKind;
    use thiserror::Error;

    type Result<T> = std::result::Result<T, ConfigError>;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub app_name: String,
        pub port: u16,
    }

    #[derive(Debug, Error)]
    pub enum ConfigError {
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        #[error("JSON parse error: {0}")]
        Parse(#[from] serde_json::Error),
    }

    // Thanks to `#[from]`, both I/O and JSON errors automatically convert into `ConfigError`—so `?` just works.
    fn load_config(path: &str) -> Result<Config> {
        let data = read_to_string(path).map_err(ConfigError::Io)?;
        let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
        Ok(cfg)
    }

    pub fn load_or_init(path: &str) -> Result<Config> {
        match load_config(path) {
            Ok(cfg) => Ok(cfg),

            // I/O: file missing → create a default and continue
            Err(ConfigError::Io(ref e)) if e.kind() == ErrorKind::NotFound => {
                let default = Config { app_name: "Demo".into(), port: 8086 };
                // Map the write error to ConfigError so `?` compiles.
                write(path, r#"{ "app_name": "Demo", "port": 8086 }"#)?;
                eprintln!("{path} not found, created with default config");
                Ok(default)
            }

            // I/O: other I/O errors → log and bubble up
            Err(ConfigError::Io(e)) => {
                eprintln!("I/O error accessing {path}: {e}");
                Err(ConfigError::Io(e))
            }

            // Parse error: file exists but JSON is invalid → log and bubble up
            Err(ConfigError::Parse(e)) => {
                eprintln!("Invalid JSON in {path}: {e}");
                Err(ConfigError::Parse(e))
            }
        }
    }
}

use my_api::load_or_init;
use std::fs::write;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
