// ex22.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex22
// cargo add serde --features derive
// cargo add serde_json

mod my_api {
    use serde::Deserialize;
    use std::fmt;
    use std::fs::{read_to_string, write};
    use std::io::ErrorKind;

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
    impl std::error::Error for ConfigError {}

    // Map the inner errors explicitly (uses map_err).
    fn load_config(path: &str) -> Result<Config, ConfigError> {
        let data = read_to_string(path).map_err(ConfigError::Io)?;
        let cfg = serde_json::from_str::<Config>(&data).map_err(ConfigError::Parse)?;
        Ok(cfg)
    }

    pub fn load_or_init(path: &str) -> Result<Config, ConfigError> {
        match load_config(path) {
            Ok(cfg) => Ok(cfg),

            // I/O: file missing → create a default and continue
            Err(ConfigError::Io(ref e)) if e.kind() == ErrorKind::NotFound => {
                let default = Config { app_name: "Demo".into(), port: 8086 };
                // Map the write error to ConfigError so `?` compiles.
                write(path, r#"{ "app_name": "Demo", "port": 8086 }"#).map_err(ConfigError::Io)?;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write("good_config.json", r#"{ "app_name": "Demo", "port": 8080 }"#)?;
    write("bad_config.json", r#"{ "app_name": "Oops", "port": "not a number" }"#)?;

    let cfg = load_or_init("bad_config.json")?;
    println!("Loaded: {} on port {}", cfg.app_name, cfg.port);
    Ok(())
}
