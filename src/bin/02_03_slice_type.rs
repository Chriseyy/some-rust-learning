// =============================================================================
// SECTION 1: THE SLICE TYPE (&str, &[T])
// -----------------------------------------------------------------------------
// A slice is a view into a contiguous sequence of elements in a collection 
// (such as a `String`, `Vec<T>`, or fixed-size array `[T; N]`).
//
// Key Properties:
// 1. Non-owning Reference: Slices DO NOT own the underlying memory.
// 2. Fat Pointer Structure: On the stack, a slice is stored as two values:
//    - Pointer: memory address of the first element in the slice.
//    - Length: the number of elements included in the slice.
// 3. Dynamically Sized View: Written as `&str` for strings or `&[T]` for collections.
// =============================================================================


// =============================================================================
// SECTION 2: THE "INDEX OUT OF SYNC" PROBLEM (WITHOUT SLICES)
// -----------------------------------------------------------------------------
// Returning a raw `usize` index instead of a string slice creates a sync bug:
// If the original `String` is cleared or modified, the stored index integer remains,
// pointing to invalid range boundaries in the modified string!
// =============================================================================

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert String to byte array slice for scanning

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // Check for space character byte representation (ASCII 32)
            return i;
        }
    }

    s.len()
}

fn second_word(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();

    let mut first_space_index = None;
    let mut second_space_index = None;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_space_index.is_none() {
                first_space_index = Some(i); // Stores index of first space
            } else {
                second_space_index = Some(i); // Stores index of second space
                break;
            }
        }
    }

    match (first_space_index, second_space_index) {
        (Some(first), Some(second)) => (first, second),
        (Some(first), None) => (first, s.len()),
        _ => (0, 0), // No spaces found
    }
}


// =============================================================================
// SECTION 3: STRING SLICE RANGE SYNTAX ([start..end])
// -----------------------------------------------------------------------------
// Range Rules:
// - `&s[0..5]`   -> Indices 0 up to (but excluding) 5.
// - `&s[..5]`    -> Equivalent to `[0..5]` (starts at beginning).
// - `&s[6..len]` -> Indices 6 up to the end.
// - `&s[6..]`    -> Equivalent to `[6..len]` (goes to the end).
// - `&s[..]`     -> Slice of the entire string.
// =============================================================================

fn string_slice_example() -> (String, String) {
    let my_string = String::from("Hello, world!");
    let slice = &my_string[0..5];   // "Hello"
    let slice2 = &my_string[7..12]; // "world"

    // Convert slices (`&str`) to owned `String` instances so they can be returned from local scope
    (slice.to_string(), slice2.to_string())
}

// Idiomatic Function Signature: Accepting `&str` instead of `&String`
// Passing `&str` allows callers to pass either a `&String` (via deref coercion)
// or a direct string literal (`&str`).
fn get_length(s: &str) -> usize {
    s.len()
}

// SOLVED WITH SLICES: The compiler ties the borrow lifetime of the returned `&str` 
// directly to the input string `s`. Modifying `s` while the slice exists is blocked!
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


// =============================================================================
// SECTION 4: GENERAL ARRAY SLICES (&[T])
// Slices work on primitive arrays (`[T; N]`) and Vectors (`Vec<T>`) just like strings.
// =============================================================================

fn slice_array(arr: &[i32]) -> &[i32] {
    &arr[1..3] // Sub-slice from index 1 up to index 3 (exclusive)
}


// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

fn main() {
    println!("=== 1. INDEX SYNC PROBLEM DEMO ===");
    let mut my_string = String::from("Hello, world!");
    let index = first_word(&my_string);
    println!("The index of the first space is: {}", index);
    println!("The first word via manual slicing is: {}", &my_string[..index]);

    // Modifying the original string invalidates manual index tracking:
    my_string.clear(); 
    // `my_string` is now empty (""), but `index` is still 6!
    // Using `&my_string[..index]` now causes a runtime PANIC (index out of bounds).
    
    println!("\n=== 2. TWO-WORD INDEX FINDER ===");
    let my_string = String::from("Hello, world!");
    let (first_space, second_space) = second_word(&my_string);
    println!("The index of the first space is: {}", first_space);
    println!("The index of the second space is: {}", second_space);

    println!("\n=== 3. STRING SLICE CREATION ===");
    let (slice1, slice2) = string_slice_example();
    println!("First slice: {}", slice1);
    println!("Second slice: {}", slice2);

    println!("\n=== 4. IDIOMATIC &str PARAMETERS ===");
    let length = get_length(&my_string);
    println!("Length of string: {}", length);

    let word = first_word_slice(&my_string);
    println!("First word via safe slice: {}", word);

    // Demonstration: Slices lock the borrowed string!
    // let mut s = String::from("hello world");
    // let word = first_word_slice(&s); // Immutable borrow (&str) created here
    // s.clear();                        // ERROR: Cannot borrow `s` as mutable because it is borrowed as immutable!
    // println!("Word: {word}");        // Immutable borrow used here

    println!("\n=== 5. ARRAY SLICES (&[T]) ===");
    let my_array = [1, 2, 3, 4, 5];
    let slice = slice_array(&my_array);
    println!("The slice of the array is: {:?}", slice);

    demo_common_slice_errors();
}


// =============================================================================
// SECTION 5: DEMONSTRATION OF SLICE COMPILER ERRORS
// Below are real compile-error cases and explanations.
// =============================================================================

fn demo_common_slice_errors() {
    println!("\n=== 6. DEMONSTRATION OF SLICE COMPILER ERRORS ===");
    println!("All compile-error snippets are documented in source comments below.");

    // Error Case 1: Returning a reference to a local variable
    // fn string_slice_broken() -> &str {
    //     let s = String::from("hello");
    //     &s[0..2] 
    // } 
    // ERROR: [E0515] returns a value referencing data owned by the current function

    // Error Case 2: Mutating data while a slice reference is active
    // let mut s = String::from("hello world");
    // let slice = &s[0..5];
    // s.clear(); 
    // println!("{}", slice);
    // ERROR: [E0502] cannot borrow `s` as mutable because it is also borrowed as immutable
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. WHAT IS A SLICE?
-------------------
- A slice (`&str` or `&[T]`) is a non-owning reference to a contiguous sequence of memory.
- Stored on the stack as a "Fat Pointer": `(pointer_to_data, length)`.


2. WHY SLICES ARE SAFER THAN MANUAL INDEXES
-------------------------------------------
- Returning raw indexes (`usize`) leaves data prone to going out of sync if the underlying 
  container is modified.
- Slices enforce compile-time borrowing rules: the compiler ensures the original data 
  CANNOT be modified or freed while a slice referencing it is active.


3. IDIOMATIC FUNCTION SIGNATURES
--------------------------------
- Prefer `fn process(s: &str)` over `fn process(s: &String)`.
- Passing `&str` accepts:
  * String slices (`&str`)
  * String literals (`"hello"`)
  * Full owned `String` variables (via automatic deref coercion: `&my_string`)


4. SLICES ON OTHER COLLECTIONS
------------------------------
- Slices work identically on primitive arrays (`&[i32]`) and `Vec<T>`.
- Example: `&array[1..3]` returns a slice referencing index 1 and index 2.
================================================================================
*/