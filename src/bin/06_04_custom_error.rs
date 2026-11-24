use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum AppError {
    EmptyInput,
    InvalidNumber(ParseIntError),
}

// Tells `?` how to automatically convert a ParseIntError into AppError.
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::InvalidNumber(err)
    }
}

// Formats the error nicely for users (required for std::error::Error).
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::EmptyInput => write!(f, "Input cannot be empty"),
            AppError::InvalidNumber(e) => write!(f, "Invalid number: {e}"),
        }
    }
}

// Marks AppError as a standard Rust Error (works with Box<dyn Error>).
impl Error for AppError {}

// `?` automatically converts ParseIntError -> AppError using the From impl above.
pub fn parse_user_port(input: &str) -> Result<u16, AppError> {
    if input.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }
    let port = input.trim().parse::<u16>()?;
    Ok(port)
}


fn main 
() -> Result<(), Box<dyn Error>> {
    // 10. CUSTOM ERROR ENUM (from src/custom_error.rs):
    println!("10. Custom error (ok)  -> {:?}", parse_user_port("9000"));
    println!("10. Custom error (err) -> {:?}", parse_user_port(""));

    Ok(())
}
