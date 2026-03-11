
// Data Types
// Rust is a statically typed language, which means that it must know the types of all variables at compile time
// Rust has a number of built-in data types, including:
// - Scalar types: integers, floating-point numbers, booleans, and characters
// - Compound types: tuples and arrays


// Integer Types
// Rust has several integer types, including:
// - i8, i16, i32, i64, i128: signed integers
// - u8, u16, u32, u64, u128: unsigned integers
// - isize, usize: signed and unsigned integers that depend on the architecture of the computer
// Length	                Signed	Unsigned
// 8-bit	                i8	    u8
// 16-bit	                i16	    u16
// 32-bit	                i32	    u32
// 64-bit	                i64	    u64
// 128-bit	                i128	u128
// Architecture-dependent	isize	usize

// Number literals	Example
// Decimal	        98_222
// Hex	            0xff
// Octal	        0o77
// Binary	        0b1111_0000
// Byte (u8 only)	b'A'


fn integer_types() {
    let a: i32 = 100;
    let b: u32 = 100;
    let c: isize = 100;
    let d: usize = 100;

    println!("a: {a}, b: {b}, c: {c}, d: {d}");

    // Overflowing
    let max_u8 = u8::MAX;
    println!("The maximum value of u8 is: {max_u8}");
    // overflowing occurs when you try to store a value that is too large for the type
    // let overflow_u8 = max_u8 + 1; // this will cause an error because the value is too large for the type u8
}

// Floating-Point Types
// Rust has two floating-point types: f32 and f64
// f32 is a 32-bit floating-point number, and f64 is a 64-bit floating-point number
fn floating_point_types() {
    let a: f32 = 3.14;
    let b: f64 = 3.14;

    println!("a: {a}, b: {b}");

    let max_f32 = f32::MAX;
    println!("The maximum value of f32 is: {max_f32}");
}


// Numeric Operations
// Rust supports the following numeric operations:
// - Addition: +
// - Subtraction: -
// - Multiplication: *
// - Division: /
// - Remainder: %
// there are more https://doc.rust-lang.org/book/appendix-02-operators.html

fn numeric_operations() {
    let a = 10;
    let b = 3;

    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let remainder = a % b;

    println!("a: {a}, b: {b}");
    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Remainder: {remainder}");
}




fn main() {
    integer_types();
    floating_point_types();
    numeric_operations();
    
}
