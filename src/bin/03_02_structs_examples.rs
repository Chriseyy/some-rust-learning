// =============================================================================
// SECTION 1: STRUCT EXAMPLE & BORROWING (&Rectangle)
// -----------------------------------------------------------------------------
// Passing a struct by reference (`&Rectangle`) lets a function read its fields
// without taking ownership. The caller can keep using the struct afterward.
//
// `#[derive(Debug)]` automatically implements the `Debug` trait so we can print
// the struct for debugging using `{:?}`, `{:#?}`, or the `dbg!` macro.
// =============================================================================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// =============================================================================
// SECTION 2: PRINTING STRUCTS WITH `Debug` ({:?} and {:#?})
// -----------------------------------------------------------------------------
// Structs do not implement `Display` (`{}`) by default because Rust cannot guess
// how you want to format them. Use `Debug` instead:
// - `{:?}`  : Compact, single-line debug output.
// - `{:#?}` : Pretty-printed, multi-line debug output (great for larger structs).
// =============================================================================

fn debug_printing_demo() {
    println!("=== 1. DEBUG PRINTING ({{:?}} vs {{:#?}}) ===");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area: {} square pixels.", area(&rect1));
    println!("Single-line Debug: {:?}", rect1);
    println!("Pretty-print Debug:\n{:#?}", rect1);
    println!("\n");
}

// =============================================================================
// SECTION 3: THE `dbg!` MACRO
// -----------------------------------------------------------------------------
// `dbg!` prints the file name, line number, expression, and value to `stderr`,
// AND returns ownership of the expression's value.
// - Tip: Pass a reference (`dbg!(&rect)`) if you don't want `dbg!` to move it.
// =============================================================================

fn dbg_macro_demo() {
    println!("=== 2. THE `dbg!` MACRO ===");

    let scale = 2;
    let rect2 = Rectangle {
        // `dbg!` evaluates `30 * scale`, prints it to stderr, and returns `60`
        width: dbg!(30 * scale),
        height: 50,
    };

    // Pass `&rect2` so `dbg!` does not take ownership of `rect2`
    dbg!(&rect2);

    // `rect2` is still valid here because we only passed a reference to `dbg!`
    println!("Area of rect2: {} square pixels.", area(&rect2));
    println!("\n");
}

// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

fn main() {
    debug_printing_demo();
    dbg_macro_demo();
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. BORROWING STRUCTS IN FUNCTIONS
---------------------------------
- Pass `&Struct` when a function only needs to read data. This keeps ownership
  in the caller so the struct remains usable after the call.
- (Next step: turning `area(&rect)` into a real method `rect.area()` using
  `impl Rectangle` is covered in `03_03_methods.rs`.)

2. DEBUGGING STRUCTS
--------------------
- Add `#[derive(Debug)]` above a struct to enable debug formatting.
- Use `{:?}` for compact output and `{:#?}` for readable multi-line output.
- Use `dbg!(&value)` for quick inspection with file and line numbers (prints to
  stderr and returns the value).
================================================================================
*/