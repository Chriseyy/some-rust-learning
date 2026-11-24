use std::error::Error;
use std::num::{IntErrorKind, ParseIntError};

// 1. BASIC MATCH: Handle Ok and Err directly.
fn parse_port_basic(input: &str) {
    match input.parse::<u16>() {
        Ok(port) => println!("1. Basic match -> Port: {port}"),
        Err(e) => println!("1. Basic match -> Invalid port: {e}"),
    }
}

// 2. SPECIFIC ERRORS (FLAT): React differently based on error.kind().
fn parse_port_with_default(input: &str) -> Result<u16, String> {
    let result = input.parse::<u16>();

    if let Ok(port) = result {
        return Ok(port);
    }

    match result.unwrap_err().kind() {
        IntErrorKind::Empty => Ok(8080), // Fallback default
        IntErrorKind::PosOverflow => Err(String::from("Port too large (max 65535)")),
        _ => Err(format!("Not a valid number: '{input}'")),
    }
}

// 3. ALTERNATIVE TO MATCH: unwrap_or_else with a closure.
fn parse_port_clean(input: &str) -> u16 {
    input.parse::<u16>().unwrap_or_else(|err| {
        if *err.kind() == IntErrorKind::Empty {
            8080
        } else {
            panic!("Fatal config error: {err}");
        }
    })
}

// 4. PANIC SHORTCUTS: unwrap() and expect().
fn panic_shortcuts() {
    let a: u16 = "3000".parse().unwrap();
    let b: u16 = "5432".parse().expect("Database port must be a valid number");
    println!("4. Shortcuts -> Ports: {a}, {b}");
}

// 5. MANUAL PROPAGATION: Pass the error back to the caller.
fn parse_and_double_manual(input: &str) -> Result<i32, ParseIntError> {
    let num = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(num * 2)
}

// 6. THE `?` OPERATOR: Short version of #5.
fn parse_and_double_short(input: &str) -> Result<i32, ParseIntError> {
    let num = input.trim().parse::<i32>()?;
    Ok(num * 2)
}

// 7. CHAINING `?`: Call the next method directly after `?` without temp variables.
fn get_raw_port(config: &str) -> Result<&str, &'static str> {
    config.split('=').nth(1).ok_or("Missing '=' in config")
}

fn parse_config_chained(config: &str) -> Result<u16, Box<dyn Error>> {
    // `?` unwraps the &str, then .trim().parse()? parses it and auto-converts the error.
    let port = get_raw_port(config)?.trim().parse::<u16>()?;
    Ok(port)
}

// 8. `?` ON OPTION & MIXING RESULT <-> OPTION:
// `.ok()` converts Result -> Option. `.ok_or()` converts Option -> Result.
fn first_valid_score(scores: &[&str]) -> Option<i32> {
    let first_text = scores.first()?; // Option<&str> -> &str
    first_text.parse::<i32>().ok()    // Result -> Option
}

fn require_first_score(scores: &[i32]) -> Result<i32, &'static str> {
    let first = scores.first().ok_or("Score list is empty")?; // Option -> Result
    Ok(*first)
}

// 9. MAIN RETURNING RESULT: Box<dyn Error> accepts any error type with `?`.
fn main() -> Result<(), Box<dyn Error>> {
    parse_port_basic("443");
    parse_port_basic("abc");

    println!("2. Specific error -> {:?}", parse_port_with_default(""));
    println!("3. unwrap_or_else -> Port: {}", parse_port_clean(""));
    panic_shortcuts();

    println!("5. Manual propagation -> {:?}", parse_and_double_manual("21"));
    println!("6. `?` operator -> Doubled: {}", parse_and_double_short(" 50 ")?);
    println!("7. Chained `?` -> Port: {}", parse_config_chained("PORT=8080")?);

    println!("8a. Result -> Option: {:?}", first_valid_score(&["95", "80"]));
    println!("8b. Option -> Result: {}", require_first_score(&[100, 90])?);

    Ok(()) // Exit code 0 (success)
}


/*
 * ============================================================================
 * RUST ERROR HANDLING CHEATSHEET: Result vs. panic! (Chapter 9.3)
 * ============================================================================
 *
 * USE `Result<T, E>` (Default choice):
 *   - Failure is expected or possible (missing file, bad user input, network down).
 *   - The caller should decide how to handle or recover from the error.
 *
 * USE `panic!` / `unwrap()` / `expect()`:
 *   - Prototypes, examples, and tests (fast to write, replace later).
 *   - Unrecoverable bugs or broken assumptions (continuing would be unsafe).
 *   - You have more info than the compiler (e.g. parsing a hardcoded valid IP):
 *     -> Use `.expect("why this cannot fail")` instead of `.unwrap()`.
 *
 * QUICK RULE OF THUMB:
 *   Expected failure?  -> Return `Result`
 *   Program bug?       -> `panic!`
 * ============================================================================
 */