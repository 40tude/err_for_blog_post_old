// ex19.rs
// CTRL+SHIFT+B to build | F5 to build and Debug | cargo run --example ex19

mod math_utils {
    // This module could be in a file math_utils.rs
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero { numerator: f64 },
        NegativeLogarithm { value: f64 },
    }

    impl std::fmt::Display for MathError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MathError::DivisionByZero { numerator } => write!(f, "Cannot divide {} by zero", numerator),
                MathError::NegativeLogarithm { value } => write!(f, "Logarithm of negative number ({})", value),
            }
        }
    }

    impl std::error::Error for MathError {}

    // Functions that return Result<_, MathError>
    pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == f64::EPSILON { Err(MathError::DivisionByZero { numerator: a }) } else { Ok(a / b) }
    }

    pub fn log10(x: f64) -> Result<f64, MathError> {
        if x < 0.0 { Err(MathError::NegativeLogarithm { value: x }) } else { Ok(x.log10()) }
    }
}

use math_utils::{divide, log10};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run() -> Result<()> {
    let my_log = log10(1024.0)?;
    println!("Log10 is {:.3}", my_log);

    let ratio = divide(10.0, 3.0)?;
    println!("Ratio is {:.3}", ratio);

    let bad_ratio = divide(5.0, 0.0)?;
    println!("This won't print because of error above ({})", bad_ratio);
    Ok(())
}

fn main() -> Result<()> {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(42);
    }
    Ok(())
}
