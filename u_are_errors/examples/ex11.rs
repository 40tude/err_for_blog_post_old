// ex11.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex11

fn first_char_upper(s: &str) -> Option<char> {
    // `first_char_upper()` returns Option<char>
    // `chars().next()` returns Option<char>
    // => we can use `?` at the end of s.chars().next()
    // If it's None, the function returns None early
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{:?}", first_char_upper("hello")); // Some('H')
    println!("{:?}", first_char_upper("")); // None
}
