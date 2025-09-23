// ex02.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex02
fn main() {
    let f = std::fs::File::open("foo.txt");
    println!("'f' after std::fs::File::open() =  {:?}", f);
}
