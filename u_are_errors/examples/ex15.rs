// ex15.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex15

use std::fs::File;
use std::io::Read;

// Read the whole file if possible, otherwise return `None`.
// We "intentionally" ignore the error details by converting Result<T, E> to Option<T>
// via `.ok()`, so that `?` can be used in the Option-returning function
fn read_file_to_string_as_option(path: &str) -> Option<String> {
    // open() returns Result<File, io::Error>. Convert to Option<File>, then `?` works with Option
    let mut file = File::open(path).ok()?;

    let mut buf = String::new();
    // `read_to_string` returns Result<usize, io::Error>. Convert to Option<usize>, , then `?` works with Option
    file.read_to_string(&mut buf).ok()?;

    Some(buf)
}

fn main() {
    let existing = "Cargo.toml";
    let missing = "_definitely_missing_.txt";

    println!("--- read_file_to_string_as_option ---");
    match read_file_to_string_as_option(existing) {
        Some(s) => println!("OK: read {} bytes from {existing}", s.len()),
        None => println!("None: could not read {existing}"),
    }
    match read_file_to_string_as_option(missing) {
        Some(s) => println!("OK: read {} bytes from {missing}", s.len()),
        None => println!("None: could not read {missing}"),
    }
}
