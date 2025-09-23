// ex04.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex04
use std::fs::File; // shortcut

fn main() {
    let result_file = File::open("foo.txt");

    match result_file {
        Ok(file) => println!("Successfully opened file: {:?}", file),
        Err(why) => panic!("Panic! opening the file: {:?}", why),
    }
}

// Feel free to comment the main() above and to play with the one below

// fn main() {
//     let result_file = File::open("foo.txt");

//     match result_file {
//         Ok(file) => println!("Successfully opened file: {:?}", file),
//         Err(why) => {
//             // panic!("Panic! opening the file: {:?}", why);
//             eprintln!("Could not open file: {:?}", why);
//         }
//     }
// }
