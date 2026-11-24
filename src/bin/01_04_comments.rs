// =============================================================================
// COMMENTS IN RUST
// -----------------------------------------------------------------------------
// Comments are ignored by the compiler, except doc comments (`///`), which
// `cargo doc` turns into HTML documentation.
//
// Three types of comments:
// 1. Line comments:  `// ...`
// 2. Block comments: `/* ... */`
// 3. Doc comments:   `/// ...` (supports Markdown)
// =============================================================================

/// Returns the user's lucky number.
///
/// # Examples
/// ```text
/// let n = get_lucky_number();
/// assert_eq!(n, 7);
/// ```
fn get_lucky_number() -> i32 {
    7
}

fn main() {
    // 1. Line comment (most common in Rust)
    let lucky_number1 = 7; // Inline comment at the end of a line

    /*
     * 2. Block comment (multi-line)
     * Useful for longer explanations or temporarily disabling a block of code.
     */
    let lucky_number2 = get_lucky_number();

    println!("Lucky number 1: {lucky_number1}");
    println!("Lucky number 2: {lucky_number2}");
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================
- Use `//` for regular code comments (idiomatic Rust prefers `//` even for
  multiple lines).
- Use `/* ... */` for large blocks or inline comments inside an expression.
- Use `///` above functions, structs, or enums to generate documentation with
  `cargo doc --open`.
================================================================================
*/