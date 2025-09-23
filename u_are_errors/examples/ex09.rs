// ex09.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex09
use std::fs::File;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>; // Type Alias

fn main() -> Result<()> {
    let file = File::open("username.txt")?;
    println!("File opened successfully: {:?}", file);
    Ok(())
}
