
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
// Length	                Signed	Unsigned    memory size
// 8-bit	                i8	    u8          1 byte
// 16-bit	                i16	    u16         2 bytes
// 32-bit	                i32	    u32         4 bytes
// 64-bit	                i64	    u64         8 bytes
// 128-bit	                i128	u128        16 bytes
// Architecture-dependent	isize	usize       4 bytes on 32-bit systems, 8 bytes on 64-bit systems

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
    println!("The size of u8 in bytes is: {}", std::mem::size_of::<u8>());
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


// -----------------------
// Boolean Type
// Rust has a boolean type called bool, which can have the values true or false
// Bool size in memory is 1 byte, but it can only have two possible values: true or false.
fn boolean_type() {
    let a: bool = true;
    let b: bool = false;    
    println!("a: {a}, b: {b}");

    // can mutate the value of a boolean variable if it is declared as mutable
    let mut c: bool = true;
    println!("c: {c}");
    c = false;
    println!("c: {c}");
    println!("The size of bool in bytes is: {}", std::mem::size_of::<bool>());
}


// -----------------------
// Character Type
// Rust has a character type called char, which represents a single Unicode scalar value
// Memory size of char is 4 bytes, because it can represent a wide range of characters from different languages and symbol sets, including emojis and other special characters.
// 4 bytes in memory = 32 bits, which allows for a large number of possible characters to be represented, including all Unicode characters. 
// This is in contrast to some other programming languages that use 1 byte (8 bits) for character representation, which can only represent a limited set of characters (e.g., ASCII).
fn character_type() {
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let cat = '😻';
    println!("c: {c}, z: {z}, cat: {cat}");
    println!("The size of char in bytes is: {}", std::mem::size_of::<char>());
}
// Note that char literals are specified with single quotes, while string literals are specified with double quotes.
// This is "a" is a string literal, while 'a' is a char literal.
// 4 bytes in memory because char is a 32-bit Unicode scalar value, which can represent a wide range of characters from different languages and symbol sets.

// -----------------------
// Compound Types
// Rust has two primitive compound types: tuples and arrays
// Tuples can group together values of different types, while arrays can only hold values of the same type.

// Tuples
// A tuple is a collection of values of different types. Tuples have a fixed length,
// and the values in a tuple can be accessed using dot notation with the index of the value.
fn tuples() {
    let tup: (i32, f64, char) = (500, 6.4, 'A');
    let (x, y, z) = tup; // destructuring a tuple into individual variables
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of the tuple is: {:?}", tup); // using debug formatting to print the entire tuple

    // can also be declarded without type annotation, and the compiler will infer the types of the values in the tuple based on the values themselves
    let tup = (500, 6.4, 'A'); // the compiler will infer that the types of the values in the tuple are i32, f64, and char, respectively, based on the values themselves
    println!("The value of the tuple is: {:?}", tup);

    let five_hundred = tup.0; // accessing the first value of the tuple
    let six_point_four = tup.1; // accessing the second value of the tuple
    let a = tup.2; // accessing the third value of the tuple
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of a is: {a}");

    // tuples can also be nested, meaning that a tuple can contain another tuple as one of its values
    let nested_tuple: ((i32, f64), char) = ((500, 6.4), 'A');
    let ((x, y), z) = nested_tuple; // destructuring a nested tuple into individual variables
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // tuple[0] this will cause an error because tuples do not support indexing like arrays
    // you can only access the values in a tuple using dot notation with the index of the value, like tup.0, tup.1, etc.
    println!("The size of the tuple in bytes is: {}", std::mem::size_of_val(&tup));
    // the size of a tuple in memory is the sum of the sizes of its individual values, plus some additional overhead for storing the tuple itself. 
    // In this case, the size of the tuple (i32, f64, char) is 4 bytes for the i32 value, 8 bytes for the f64 value, and 4 bytes for the char value, plus some additional overhead for storing the tuple itself, resulting in a total size of 16 bytes in memory.
    // in memory it is stored as a contiguous block of memory, with the values of the tuple stored in the order they are declared.
}

// Arrays
// An array is a collection of values of the same type. Arrays have a fixed length, and the values in an array can be accessed using indexing with square brackets.
fn arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // an array of 5 integers
    let b: [f64; 3] = [3.14, 2.71, 1.41]; // an array of 3 floating-point numbers
    let c: [char; 4] = ['a', 'b', 'c', 'd']; // an array of 4 characters
    // the type of an array is specified as [T; N], where T is the type of the elements in the array and N is the number of elements in the array
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);

    // can also be declared without type annotation, and the compiler will infer the types of the values in the array based on the values themselves
    let a = [1, 2, 3, 4, 5]; // the compiler will infer that the type of the values in the array is i32 based on the values themselves
    let b = [3.14, 2.71, 1.41]; // the compiler will infer that the type of the values in the array is f64 based on the values themselves
    let c = ['a', 'b', 'c', 'd']; // the compiler will infer that the type of the values in the array is char based on the values themselves
    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);

    // accessing the values in an array using indexing with square brackets
    println!("The first element of a is: {}", a[0]);
    println!("The second element of b is: {}", b[1]);
    println!("The third element of c is: {}", c[2]);
    // arrays can also be nested, meaning that an array can contain another array as one of its values
    let nested_array: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]]; // an array of 2 arrays, each containing 3 integers
    println!("nested_array: {:?}", nested_array);
    println!("The first element of the first array in nested_array is: {}", nested_array[0][0]);
    println!("The second element of the second array in nested_array is: {}", nested_array[1][1]);
    // arrays do not support indexing with negative numbers, so nested_array[-1][0] will cause an error because it is trying to access the last element of the first array in nested_array, which does not exist. 
    // you can only access the values in an array using positive indexing with square brackets, like nested_array[0][0], nested_array[1][1], etc.
    // can not call a.1 because arrays do not support tuple-like indexing with dot notation.

    println!("The size of the array in bytes is: {}", std::mem::size_of_val(&a));
    // the size of an array in memory is the size of each element multiplied by the number of elements in the array, plus some additional overhead for storing the array itself. 
    // In this case, the size of the array [i32; 5] is 4 bytes for each i32 value, multiplied by 5 elements, plus some additional overhead for storing the array itself, resulting in a total size of 20 bytes in memory.
    // in memory, arrays are stored as a contiguous block of memory, with the values of the array stored in the order they are declared.

    // memory size printing for nested arrays
    println!("The size of the nested array in bytes is: {}", std::mem::size_of_val(&nested_array));
    // the size of the nested array [[i32; 3]; 2] is 4 bytes for each i32 value, multiplied by 3 elements in
    // each inner array, multiplied by 2 inner arrays, plus some additional overhead for storing the nested array itself, resulting in a total size of 48 bytes in memory.

}   

fn main() {
    integer_types();
    floating_point_types();
    numeric_operations();
    boolean_type();
    character_type();
    tuples();
    arrays();
}



// Typical errors:
// let x = 5; // this is an immutable variable, we can not change its value
// x = 6; // this will cause an error because x is immutable    

// let y: i32; // this is a variable declaration without initialization, we must initialize it before using it
// println!("The value of y is: {y}"); // this will cause an error because y is not initialized

// array out of range error
// let a = [1, 2, 3, 4, 5];
// println!("The sixth element of a is: {}", a[5]); // this will cause an error because there is no sixth element in the array a, which has only 5 elements (index 0 to 4)

// Tuple out of range error
// let tup = (500, 6.4, 'A');
// println!("The fourth element of tup is: {}", tup.3); // this will cause an error because there is no fourth element in the tuple tup, which has only 3 elements (index 0 to 2)
