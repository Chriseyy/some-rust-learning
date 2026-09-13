// =============================================================================
// SECTION 1: BASIC FUNCTIONS & DECLARATIONS
// In Rust, functions are declared using the `fn` keyword.
// - Function names use snake_case by default.
// - Functions can be defined anywhere in the module (above or below `main`),
//   as long as they are within a reachable scope.
// =============================================================================

fn another_function() {
    println!("Another function.");
}
// Explanation:
// A function definition consists of `fn`, the function identifier, parentheses `()`,
// and a block of code enclosed within curly braces `{}`.


// =============================================================================
// SECTION 2: FUNCTIONS WITH PARAMETERS
// Functions can accept arguments. In Rust, you MUST explicitly declare the type
// of every parameter in a function signature.
// =============================================================================

fn another_function_with_parameters(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn some_random_text_with_parameters(text: &str) {
    println!("This is some random text: {}", text);
}
// Explanation:
// - Parameters are values passed into a function when it is called.
// - Every parameter requires a type annotation (`x: i32`, `text: &str`).
// - Rust does not perform type inference for function parameters—this design decision
//   ensures that functions serve as clear, explicit contracts.
// - Parameters are immutable by default inside the function body unless marked `mut`
//   (e.g., `fn process(mut data: String)`).


// =============================================================================
// SECTION 3: STATEMENTS VS. EXPRESSIONS
// Rust is primarily an expression-based language.
// - Statement: An instruction that performs an action and does NOT return a value.
//              Always ends with a semicolon `;`. (e.g., `let x = 5;`).
// - Expression: Evaluates to a resulting value. Does NOT end with a semicolon.
//              (e.g., `5 + 6`, block expressions `{ let x = 3; x + 1 }`).
// =============================================================================

fn add_stuff() {
    println!("=== STATEMENTS & EXPRESSIONS DEMO ===");
    
    // A block `{ ... }` is an expression that evaluates to its last expression (`x + 1`)
    let y = {
        let x = 3;
        x + 1 // Note: No semicolon! This line evaluates to 4.
    };

    println!("The value of y is: {y}");
    println!("\n");
}


// =============================================================================
// SECTION 4: RETURN VALUES & IMPLICIT/EXPLICIT RETURNS
// Functions can return values to the caller using the arrow syntax `-> Type`.
// - Implicit Return (Idiomatic Rust): The final expression in a function body 
//   (without a semicolon) is automatically returned.
// - Explicit Return: Using the `return` keyword, useful for returning early.
// =============================================================================

fn number_plus_one(x: i32) -> i32 {
    x + 1 // Implicit return: No semicolon at the end!
}

// Adding a semicolon turns `x + 1` into a statement, causing it to return `()` (unit type).
// fn number_plus_one_invalid(x: i32) -> i32 {
//     x + 1; // ERROR: expected `i32`, found `()`
// }

fn check_age(age: i32) -> String {
    // Early return if the condition is met
    if age < 18 {
        return String::from("TOOO YOUNG"); 
    }
    
    println!("Age passed");
    // Final expression (implicit return)
    String::from("HELLLLOO!") 
}

fn return_string_example() -> String {
    String::from("Hello, world!")
}


// =============================================================================
// SECTION 5: RETURNING RETURNING COMPLEX TYPES (TUPLES & STRUCTS)
// Functions can return complex structures, including Tuples, Structs, Enums, etc.
// =============================================================================

fn get_user_info() -> (String, i32) {
    let name = String::from("Alice");
    let age = 30;
    (name, age) // Returns a tuple containing (String, i32)
}

struct User {
    name: String,
    age: i32,
}

fn create_user() -> User {
    User {
        name: String::from("Chris"),
        age: 30,
    }
}


// =============================================================================
// SECTION 6: EARLY RETURNS IN UNIT FUNCTIONS (`-> ()`)
// Functions without an explicit return type return the unit type `()`.
// You can use a bare `return;` statement to exit early from a `()` function.
// =============================================================================

fn say_hello(name: &str) { 
    if name == "" {
        println!("No name provided!");
        return; // Exits early without executing the remaining lines
    }
    
    println!("Hello, {}!", name);
}


// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

fn main() {
    println!("=== 1. CALLING BASIC FUNCTIONS ===");
    another_function();
    another_function_with_parameters(5, 10);
    some_random_text_with_parameters("Hello, world!");
    println!("\n");

    add_stuff();

    println!("=== 2. CALLING RETURN FUNCTIONS ===");
    let result = number_plus_one(5);
    println!("The result is: {result}");

    // Calling functions and dropping the return value
    number_plus_one(1); 
    return_string_example(); 
    println!("\n");

    println!("=== 3. EARLY RETURNS & COMPLEX TYPES ===");
    let age = 17;
    let age_check = check_age(age);
    println!("Age check: {age_check}");

    let str2 = return_string_example();
    println!("The value of str is: {str2}");

    let user = create_user();
    println!("User name: {}, User age: {}", user.name, user.age);

    // Tuple returns and destructuring
    let user_info = get_user_info();  
    let (name, age) = get_user_info(); // Unpacking tuple into separate variables
    println!("User info tuple: {:?}", user_info);
    println!("Destructured Name: {}, Age: {}", name, age);
    println!("\n");

    println!("=== 4. EARLY RETURN UNIT DEMO ===");
    say_hello("");     // Triggers early return
    say_hello("Alice"); // Completes full function body
    println!("\n");

    demo_common_compiler_errors();
}


// =============================================================================
// SECTION 7: COMMON FUNCTION COMPILER ERRORS (DEMONSTRATION)
// Below are real examples of common mistakes and how the Rust compiler handles them.
// =============================================================================

fn demo_common_compiler_errors() {
    println!("=== 5. DEMONSTRATION OF COMPILER ERRORS ===");
    println!("Compile-error examples are documented in source comments below.");

    // Error 1: Missing Parameter Type Annotations
    // fn stuff2(x) { println!("{x}"); } 
    // ERROR: expected type, found `)` -> Type annotations ARE REQUIRED for all parameters.

    // Error 2: Mismatched Return Type
    // fn get_name() -> String {
    //     42 
    // } 
    // ERROR: expected `String`, found integer.

    // Error 3: Returning a value when return type defaults to `()`
    // fn get_number() { // Implicitly returns `()`
    //     5 
    // } 
    // ERROR: expected `()`, found integer.

    // Error 4: Accidental Semicolon on Final Expression
    // fn add_one(x: i32) -> i32 {
    //     x + 1; // Adding `;` converts this expression into a statement returning `()`
    // } 
    // ERROR: expected `i32`, found `()`.
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. EXPRESSIONS VS. STATEMENTS IN RUST
-------------------------------------
- Expressions return a value and DO NOT end with a semicolon `;`.
- Statements perform an action and DO end with a semicolon `;`.
- A block `{ ... }` evaluates to its last expression. This is heavily used in 
  `let` assignments, `match` arms, and function return values.


2. IMPLICIT RETURNS VS. EXPLICIT RETURNS
----------------------------------------
- Idiomatic Rust prefers implicit returns (omitting `return` and `;` on the 
  last expression of a function).
- Use explicit returns (`return value;`) primarily for early exits (e.g., inside 
  guard clauses or validation logic).


3. TYPE ANNOTATIONS IN FUNCTION SIGNATURES
------------------------------------------
- Rust requires explicit type annotations for all function parameters and 
  return types (except when returning the unit type `()`).
- This guarantees stable contracts: changing internal function logic won't 
  accidentally change the function's public interface or break callers.
================================================================================
*/