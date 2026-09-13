// =============================================================================
// SECTION 1: VARIABLE DECLARATION & MUTABILITY BASICS
// Variables in Rust are immutable by default (`let`). To make a variable
// mutable, you must explicitly add the `mut` keyword (`let mut`).
// You can also add explicit type annotations, e.g., `: i32`.
// =============================================================================

fn start_idea() {
    println!("=== 1. START IDEA & MUTABILITY BASICS ===");

    // Immutable variable: value cannot be changed after declaration
    let x = 5; 
    println!("The value of x is: {x}");

    // Mutable variable: value can be updated later
    let mut y = 10; 
    println!("The value of y is: {y}");
    y = 15; // Reassigning a new value to y
    println!("The value of y is: {y}");

    // Explicit type annotation: i32 is a signed 32-bit integer (occupies 4 bytes in memory)
    let z: i32 = 20; 
    println!("The value of z is: {z}");
    // z = 25; // Error: Cannot assign twice to immutable variable `z`

    // Uninitialized variable: must specify type annotation upfront before assigning later
    let a: i32; 
    a = 30; // Deferred initialization
    println!("The value of a is: {a}");
    println!("\n");
}

fn variab_mut() {
    println!("=== 2. MUTABILITY DEMO ===");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("\n");
}
// Explanation: Without `let mut`, we cannot change the value of x because variables are
// immutable by default. Attempting to reassign `x` without `mut` causes a compilation error.


// =============================================================================
// SECTION 2: DECLARING CONSTANTS
// Constants are declared using `const` instead of `let`.
// Key rules for `const`:
// 1. Must always have an explicit type annotation (e.g., `: u32`).
// 2. Can be declared in any scope, including global module scope.
// 3. Must be set to a compile-time constant expression (not dynamic runtime values).
// 4. Are strictly immutable and can NEVER use `mut`.
// =============================================================================

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
const MAX_POINTS: u32 = 100_000;

fn declaing_const() {
    println!("=== 3. CONSTANTS DEMO ===");
    {
        let x = THREE_HOURS_IN_SECONDS + MAX_POINTS;
        println!("The value of x (constant sum) is: {x}");
    }
    println!("\n");
}
// Explanation: Constants must always specify a type and cannot infer types from expressions.
// They remain valid throughout the entire execution of the program.


// =============================================================================
// SECTION 3: SHADOWING
// Shadowing allows declaring a new variable with the exact same name as a previous one.
// Difference from `mut`: 
// 1. Shadowing creates a NEW variable (can change the data type).
// 2. Requires using the `let` keyword again.
// 3. Shadowed variables in inner scopes go out of scope when the block ends.
// =============================================================================

fn shadowing() {
    println!("=== 4. BASIC SHADOWING ===");
    let x = 5;

    // Shadowing `x` in the outer scope (x becomes 6)
    let x = x + 1;

    {
        // Shadowing `x` inside an inner block scope (x becomes 12)
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    // The inner `x` goes out of scope here; outer `x` (6) is visible again
    println!("The value of x is: {x}");
    println!("\n");
}
// Explanation: Curly brackets `{ }` create an isolated scope.
// The outer `x` has value 6. Inside `{ }`, `let x` creates a second inner `x` (value 12).
// As long as execution is inside the inner block, the inner `x` shadows the outer `x`.
// Once the inner block terminates at `}`, the inner `x` is destroyed, and outer `x` (6) returns.

fn shadowing2() {
    println!("=== 5. ADVANCED SHADOWING USE CASES ===");

    // Use Case A: Transforming data types without creating messy variable names
    // Without Shadowing (requires distinct variable names):
    let eingabe_string = "42";
    let eingabe_getrimmt = eingabe_string.trim();
    let eingabe_zahl: i32 = eingabe_getrimmt.parse().unwrap();
    println!("Die Zahl ist ohne Shadowing: {eingabe_zahl}");

    // With Shadowing (reuses the `eingabe` identifier cleanly while changing types):
    let eingabe = "42";                 // &str
    let eingabe = eingabe.trim();       // &str
    let eingabe: i32 = eingabe.parse().unwrap(); // i32
    println!("Die Zahl ist mit Shadowing: {eingabe}");

    // Use Case B: Locking down mutability ("Freezing" a variable)
    let mut passwort = "geheim";
    println!("Das Passwort ist: {passwort}");
    
    // Simulate mutation/processing...
    // passwort = "new_encrypted_value";

    // Re-declaring `passwort` with `let` (without `mut`) locks it down.
    // It removes future mutability while keeping the final processed value.
    let passwort = "passwort123";
    // passwort = "other"; // Error! `passwort` is now immutable again.
    println!("Das finale (unveränderliche) Passwort ist: {passwort}");
    println!("\n");
}


// =============================================================================
// MAIN ENTRY POINT
// Guaranteed to run without errors or warnings.
// =============================================================================

fn main() {
    start_idea();
    variab_mut();
    declaing_const();
    shadowing();
    shadowing2();
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. WHEN TO USE `let` (IMMUTABLE BY DEFAULT)
-------------------------------------------
Use immutable variables (`let`) for most values in your code. Immutable variables 
prevent accidental state changes, making concurrent code safe and logic predictable.


2. WHEN TO USE `let mut` (MUTABLE VARIABLES)
--------------------------------------------
Use `let mut` when a value needs to change in-place within its lifecycle (e.g., 
loop counters, accumulating buffers, building dynamic arrays/vectors).

Pros:
+ Performance: Avoids re-allocating new memory when updating data in-place.
+ Convenience: Allows straightforward mutation inside loops or state updates.

Cons:
- Bug Risk: Increases the risk of logic bugs if state changes unexpectedly.


3. WHEN TO USE `const` (CONSTANTS)
----------------------------------
Use `const` for fixed global or scope-wide values known at compile time that 
NEVER change (e.g., maximum limits, conversion rates, configuration constants).

Pros:
+ Inlined by the compiler (zero runtime cost).
+ Can be declared globally outside functions.

Cons:
- Cannot hold values evaluated at runtime (e.g., return value of a function call).
- Must explicitly declare type annotations (`: Type`).


4. WHEN TO USE SHADOWING
------------------------
Use shadowing (`let x = x + 1;` or `let x: i32 = x.parse();`) when:
- Transforming a value's type or structure while maintaining a clean concept name.
- Locking down a mutable variable after initialization so it can't be changed later.

Pros:
+ Avoids invented variable names like `input_str`, `input_trimmed`, `input_int`.
+ Preserves immutability guarantees after processing finishes.

Cons:
- Can cause confusion if overused across long functions where readers miss 
  that a variable was re-declared.
================================================================================
*/