// =============================================================================
// SECTION 1: DATA TYPES OVERVIEW & SCALAR TYPES
// Rust is a statically typed language: every variable's type must be known at
// compile time (either via explicit annotation or compiler inference).
// Primitive Data Types fall into two primary categories:
// - Scalar types: represent a single value (Integers, Floats, Booleans, Chars).
// - Compound types: group multiple values into one type (Tuples, Arrays).
// =============================================================================


// =============================================================================
// SECTION 2: INTEGER TYPES & LITERALS
//
// Length                  Signed   Unsigned    Memory Size
// 8-bit                   i8       u8          1 byte
// 16-bit                  i16      u16         2 bytes
// 32-bit                  i32      u32         4 bytes (default integer type)
// 64-bit                  i64      u64         8 bytes
// 128-bit                 i128     u128        16 bytes
// Arch-dependent          isize    usize       4 bytes (32-bit sys) / 8 bytes (64-bit sys)
//
// Number Literals Examples:
// Decimal:                98_222
// Hexadecimal:            0xff
// Octal:                  0o77
// Binary:                 0b1111_0000
// Byte (u8 only):         b'A'
// =============================================================================

fn integer_types() {
    println!("=== 1. INTEGER TYPES & OVERFLOW ===");

    let a: i32 = 100;
    let b: u32 = 100;
    let c: isize = 100;
    let d: usize = 100;

    println!("a: {a}, b: {b}, c: {c}, d: {d}");

    // Integer Overflow Demo
    let max_u8 = u8::MAX;
    println!("The maximum value of u8 is: {max_u8}");
    
    // Overflowing occurs when you try to store a value larger than the type allows.
    // In debug mode, Rust panics at runtime on overflow.
    // In release mode (--release), Rust performs two's complement wrapping.
    // let overflow_u8 = max_u8 + 1; // Un-commenting causes a compile/runtime panic
    
    println!("The size of u8 in bytes is: {}", std::mem::size_of::<u8>());
    println!("\n");
}


// =============================================================================
// SECTION 3: FLOATING-POINT TYPES
// Rust offers two floating-point primitive types:
// - `f32`: Single-precision float (32 bits / 4 bytes)
// - `f64`: Double-precision float (64 bits / 8 bytes) - Default choice due to precision
// =============================================================================

fn floating_point_types() {
    println!("=== 2. FLOATING-POINT TYPES ===");

    let a: f32 = 3.14;
    let b: f64 = 3.14; // Default float type in Rust

    println!("a: {a}, b: {b}");

    let max_f32 = f32::MAX;
    println!("The maximum value of f32 is: {max_f32}");
    println!("The size of f32 in bytes is: {}", std::mem::size_of::<f32>());
    println!("The size of f64 in bytes is: {}", std::mem::size_of::<f64>());
    println!("\n");
}


// =============================================================================
// SECTION 4: NUMERIC OPERATIONS
// Basic arithmetic operators supported in Rust:
// Addition (+), Subtraction (-), Multiplication (*), Division (/), Remainder (%)
// =============================================================================

fn numeric_operations() {
    println!("=== 3. NUMERIC OPERATIONS ===");

    let a = 10;
    let b = 3;

    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;     // Integer division truncates towards zero (10 / 3 = 3)
    let remainder = a % b;    // Modulo/Remainder operation (10 % 3 = 1)

    println!("a: {a}, b: {b}");
    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Remainder: {remainder}");
    println!("\n");
}


// =============================================================================
// SECTION 5: BOOLEAN TYPE
// Repesents truth values: `true` or `false`.
// Memory footprint: exactly 1 byte.
// =============================================================================

fn boolean_type() {
    println!("=== 4. BOOLEAN TYPE ===");

    let a: bool = true;
    let b: bool = false;    
    println!("a: {a}, b: {b}");

    // Mutating a boolean variable
    let mut c: bool = true;
    println!("c (initial): {c}");
    c = false;
    println!("c (mutated): {c}");

    println!("The size of bool in bytes is: {}", std::mem::size_of::<bool>());
    println!("\n");
}


// =============================================================================
// SECTION 6: CHARACTER TYPE (UNICODE)
// Represented by `char`.
// Memory footprint: 4 bytes (32-bit Unicode Scalar Value).
// Unlike 1-byte ASCII chars in C/C++, Rust's `char` can hold emojis, accented
// characters, Japanese/Chinese characters, and mathematical symbols.
// =============================================================================

fn character_type() {
    println!("=== 5. CHARACTER TYPE ===");

    let c = 'z';
    let z: char = 'Z'; // Explicit type annotation
    let cat = '😻';   // Emoji stored inside a primitive char!

    println!("c: {c}, z: {z}, cat: {cat}");
    println!("The size of char in bytes is: {}", std::mem::size_of::<char>());
    println!("\n");
}
// Note:
// 'a' (single quotes)  -> char literal (4 bytes)
// "a" (double quotes)  -> string slice literal (&str)


// =============================================================================
// SECTION 7: COMPOUND TYPES - TUPLES
// Tuples group together values of DIFFERENT types into a single compound value.
// Features:
// - Fixed size: cannot grow or shrink once declared.
// - Destructuring: unpack using `let (x, y, z) = tuple;`
// - Direct access: use dot notation with zero-based index (`tup.0`, `tup.1`).
// =============================================================================

fn tuples() {
    println!("=== 6. TUPLE COMPOUND TYPE ===");

    let tup: (i32, f64, char) = (500, 6.4, 'A');
    let (x, y, z) = tup; // Destructuring into individual variables
    println!("Destructured x: {x}");
    println!("Destructured y: {y}");
    println!("Destructured z: {z}");
    println!("Full tuple via debug format: {:?}", tup);

    // Type inference example
    let tup = (500, 6.4, 'A'); // Compiler infers (i32, f64, char)
    println!("Inferred tuple: {:?}", tup);

    // Accessing elements using dot index notation
    let five_hundred = tup.0; 
    let six_point_four = tup.1; 
    let a = tup.2; 
    println!("Dot access tup.0: {five_hundred}");
    println!("Dot access tup.1: {six_point_four}");
    println!("Dot access tup.2: {a}");

    // Nested tuple example
    let nested_tuple: ((i32, f64), char) = ((500, 6.4), 'A');
    let ((nx, ny), nz) = nested_tuple; // Nested destructuring
    println!("Nested x: {nx}, y: {ny}, z: {nz}");

    // Note: `tup[0]` is invalid syntax for tuples! Use `tup.0` instead.
    println!("The size of tuple in bytes is: {}", std::mem::size_of_val(&tup));
    // Memory footprint: stored contiguously with byte alignment padding.
    println!("\n");
}


// =============================================================================
// SECTION 8: COMPOUND TYPES - ARRAYS
// Arrays group together values of the SAME type into fixed-size contiguous memory.
// Type notation: `[T; N]` where T is the element type and N is the length.
// Features:
// - Fixed length: allocated on the stack.
// - Indexing: accessed via square brackets (`arr[0]`).
// - Bounds checking: Rust performs runtime checks to prevent buffer overflows.
// =============================================================================

fn arrays() {
    println!("=== 7. ARRAY COMPOUND TYPE ===");

    let a: [i32; 5] = [1, 2, 3, 4, 5];            // Array of 5 integers
    let b: [f64; 3] = [3.14, 2.71, 1.41];         // Array of 3 floats
    let c: [char; 4] = ['a', 'b', 'c', 'd'];      // Array of 4 characters
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);

    // Type inference examples
    let a = [1, 2, 3, 4, 5];
    let b = [3.14, 2.71, 1.41];
    let c = ['a', 'b', 'c', 'd'];
    println!("Inferred arrays: a: {:?}, b: {:?}, c: {:?}", a, b, c);

    // Accessing elements using index notation `[index]`
    println!("First element of a: {}", a[0]);
    println!("Second element of b: {}", b[1]);
    println!("Third element of c: {}", c[2]);

    // Repeating value shorthand syntax: [value; count]
    let repeated_array = [3; 5]; // Produces [3, 3, 3, 3, 3]
    println!("Repeated array shorthand: {:?}", repeated_array);

    // Nested array example
    let nested_array: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]]; 
    println!("nested_array: {:?}", nested_array);
    println!("nested_array[0][0]: {}", nested_array[0][0]);
    println!("nested_array[1][1]: {}", nested_array[1][1]);

    // Notes:
    // - Negative indexing (e.g., `arr[-1]`) is not supported in Rust.
    // - Dot notation (e.g., `a.1`) is invalid for arrays.

    println!("The size of array [i32; 5] in bytes is: {}", std::mem::size_of_val(&a));
    println!("The size of nested_array in bytes is: {}", std::mem::size_of_val(&nested_array));
    println!("\n");
}


// =============================================================================
// MAIN ENTRY POINT
// Guaranteed to run cleanly without errors.
// =============================================================================

fn main() {
    integer_types();
    floating_point_types();
    numeric_operations();
    boolean_type();
    character_type();
    tuples();
    arrays();
    demo_common_compiler_errors();
}


// =============================================================================
// SECTION 9: COMMON COMPILER ERRORS (DEMONSTRATION)
// Below are real examples of invalid code and why the Rust compiler rejects them.
// =============================================================================

fn demo_common_compiler_errors() {
    println!("=== 8. DEMONSTRATION OF COMMON ERRORS ===");
    println!("All compile-error snippets are documented in source comments below.");

    // Error 1: Reassigning an immutable variable
    // let x = 5;
    // x = 6; // ERROR: [E0384] Cannot assign twice to immutable variable `x`

    // Error 2: Using an uninitialized variable
    // let y: i32;
    // println!("The value of y is: {y}"); // ERROR: [E0381] Used uninitialized variable `y`

    // Error 3: Indexing an array out of bounds
    // let a = [1, 2, 3, 4, 5];
    // println!("{}", a[5]); // ERROR/PANIC: Index out of bounds (index is 5, but len is 5)

    // Error 4: Accessing a non-existent tuple element
    // let tup = (500, 6.4, 'A');
    // println!("{}", tup.3); // ERROR: [E0609] No field `3` on type `(i32, f64, char)`
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. SCALAR vs. COMPOUND TYPES
----------------------------
- Scalar: Holds a single primitive value.
  * Integers (`i32`, `u32`, `usize`, etc.)
  * Floats (`f32`, `f64`)
  * Booleans (`bool`)
  * Characters (`char` -> 4-byte Unicode scalar)

- Compound: Groups multiple values into one variable on the stack.
  * Tuples (`(T1, T2)`): Can hold DIFFERENT types. Access via `.0`, `.1`.
  * Arrays (`[T; N]`): Must hold the SAME type. Fixed length `N`. Access via `[0]`.


2. ARRAYS vs. VECTORS (`Vec<T>`)
--------------------------------
- Use Arrays (`[T; N]`) when data length is FIXED and KNOWN at compile time,
  and you want low-overhead allocation directly on the stack (e.g., months in a year).
- Use Vectors (`Vec<T>`) when data length needs to GROW or SHRINK dynamically at runtime
  (allocated on the heap).


3. MEMORY SAFETY & BOUNDS CHECKING
----------------------------------
- Rust checks array indices at runtime (or compile time if constant). Accessing an index 
  beyond the array bounds causes an immediate panic rather than exposing security 
  vulnerabilities or corrupting memory like in C/C++.
================================================================================
*/