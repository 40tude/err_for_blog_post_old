// ex07.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex07
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // if Err, returns Err up
    let mut username = String::new();
    file.read_to_string(&mut username)?; // if Err, returns Err up
    Ok(username) // if we got here, all good
}

fn main() {
    // Use the function and handle any error here
    match read_username_from_file() {
        Ok(name) => println!("Username: {name}"),
        Err(e) => eprintln!("Error reading username: {e}"),
    }
}
