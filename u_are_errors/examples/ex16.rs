// ex16.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex16

use std::fs::File;
use std::io::Read;

fn read_with_logging(path: &str) -> Option<String> {
    let mut file = File::open(path)
        .map_err(|e| {
            eprintln!("[read_with_logging] open error: {e}");
            e
        })
        .ok()?; // Result<File, io::Error> -> Option<File>

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|e| {
            eprintln!("[read_with_logging] read error: {e}");
            e
        })
        .ok()?; // Result<usize, io::Error> -> Option<usize>

    Some(buf)
}

fn main() {
    let existing = "Cargo.toml";
    let missing = "_definitely_missing_.txt";

    match read_with_logging(existing) {
        Some(s) => println!("OK: read {} bytes from {existing}", s.len()),
        None => println!("None: could not read {existing}"),
    }
    match read_with_logging(missing) {
        Some(s) => println!("OK: read {} bytes from {missing}", s.len()),
        None => println!("None: could not read {missing}"),
    }
}
