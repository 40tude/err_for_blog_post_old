// ex08.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex08
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("username.txt")?; // if this errors, `main()` will return Err
    println!("File opened successfully: {:?}", file);
    Ok(())
}
