// ex12.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex12

// ! DOES NOT COMPILE

use std::fs::File;

fn bad_example() -> Option<File> {
    // `File::open` returns Result<File, io::Error>
    // But our function returns Option<File>.
    // The compiler rejects this because it cannot convert Result into Option automatically.
    let file = File::open("username.txt")?;
    Some(file)
}

fn main() {
    let f = bad_example();
    println!("{:?}", f);
}
