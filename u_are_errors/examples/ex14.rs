// ex14.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex14

// Using Option: No error message, just None if invalid.
fn safe_divide_option(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // indicate failure without detail
    } else {
        Some(a / b)
    }
}

// Using Result: Provide an error message on failure.
fn safe_divide_result(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("Division by zero") // error string explaining the issue
    } else {
        Ok(a / b)
    }
}

fn main() {
    let x = safe_divide_option(10.0, 0.0);
    let y = safe_divide_result(10.0, 0.0);
    println!("Option version: {:?}", x); // None
    println!("Result version: {:?}", y); // Err(Division by zero)
}
