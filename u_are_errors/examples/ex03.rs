// ex03.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex03
fn main() {
    let f: std::fs::File = std::fs::File::open("foo.txt");
    println!("{:?}", f);
}
