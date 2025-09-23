// ex10.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex10

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_number(s: &str) -> Result<i32> {
    // `parse::<i32>()` returns Result<i32, ParseIntError>
    // The `?` operator works here because ParseIntError implements
    // the `Error` trait, and Rust knows how to convert it into Box<dyn Error>.
    let n: i32 = s.parse()?;
    Ok(n)
}

fn main() -> Result<()> {
    let value = parse_number("123sdfsdf")?;
    println!("Parsed value: {value}");
    Ok(())
}
