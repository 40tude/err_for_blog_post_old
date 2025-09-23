// ex06.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex06
fn main() {
    let some_number: Option<i32> = Some(42);
    let none_number: Option<i32> = None;

    // unwrap_or_else takes a closure that computes a fallback value
    println!(
        "Some(42).unwrap_or_else(...) = {}",
        some_number.unwrap_or_else(|| {
            println!("Closure not called, since we had Some");
            0
        })
    );

    println!(
        "None::<i32>.unwrap_or_else(...) = {}",
        none_number.unwrap_or_else(|| {
            println!("Closure called, computing fallback value...");
            100
        })
    );
}
