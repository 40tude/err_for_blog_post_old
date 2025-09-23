// ex05.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex05
use std::fs::File; // shortcut
use std::io::Read;

fn main() {
    let result_file = File::open("foo.txt");
    let mut bob = match result_file {
        Ok(alice) => alice,
        Err(why) => panic!("Panic! opening the file: {:?}", why),
    };
    println!("{:?}", bob);

    let mut s = String::new();
    match bob.read_to_string(&mut s) {
        Ok(_) => print!("Content:\n{}", s),
        Err(why) => panic!("Panic! reading the file: {:?}", why),
    }
}
