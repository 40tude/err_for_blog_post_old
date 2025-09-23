// ex13.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex13

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_first_char(s: &str) -> Result<char> {
    // Convert Option<char> into Result<char, String>
    s.chars().next().ok_or("String was empty".into())
}

fn main() -> Result<()> {
    let c1 = get_first_char("hello")?;
    println!("First char: {c1}");

    let c2 = get_first_char("")?; // This will return Err
    println!("First char: {c2}");

    Ok(())
}
