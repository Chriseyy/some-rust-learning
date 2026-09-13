// =============================================================================
// SECTION 1: IMMUTABLE REFERENCES & BORROWING (&T)
// -----------------------------------------------------------------------------
// Instead of taking ownership of a value (which moves it and invalidates the original
// variable), we can pass a REFERENCE using the `&` operator.
//
// Key Concepts:
// - Borrowing: Accessing a value via a reference without taking ownership.
// - Guaranteed Validity: References in Rust are guaranteed to point to valid memory.
// - Dereferencing: The opposite of `&` is `*`, used to access the inner value.
// =============================================================================

fn refernce_string() {
    println!("=== 1. IMMUTABLE REFERENCE (&String) ===");

    let s1 = String::from("hello");

    // We pass `&s1` (a reference) into `calculate_length`.
    // `s1` remains valid in this scope after the function call!
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    println!("\n");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` goes out of scope here. Because `s` only borrowed the data (it did not own it),
  // the original `String` is NOT dropped or freed.


// =============================================================================
// SECTION 2: MUTABLE REFERENCES (&mut T)
// -----------------------------------------------------------------------------
// References are immutable by default (`&T`). You cannot modify data through an
// immutable reference. To mutate borrowed data, you must explicitly use `&mut T`.
//
// Requirements for Mutable Borrowing:
// 1. The original variable MUST be declared as `mut` (`let mut s = ...`).
// 2. The reference MUST be declared as `&mut s`.
// 3. The function parameter MUST expect `&mut String`.
// =============================================================================

fn mutable_references_1() {
    println!("=== 2. MUTABLE REFERENCES (&mut String) ===");
    let mut s = String::from("hello");

    // Pass an explicit mutable reference
    change(&mut s);

    println!("Mutated string after function call: {s}");
    println!("\n");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Modifies the borrowed String in-place
}

// Scoping Mutable References:
// You can have multiple mutable references sequentially, as long as their lifetimes 
// do NOT overlap.
fn mutable_references_2() {
    println!("=== 3. SCOPED MUTABLE REFERENCES ===");
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        print!("{r1} ");
    } // `r1` goes out of scope here! Its exclusive borrow ends.

    let r2 = &mut s; // Valid! No other active borrow exists.
    println!("{r2}");
    println!("\n");
}


// =============================================================================
// SECTION 3: THE GOLDEN BORROWING RULES & NON-LEXICAL LIFETIMES (NLL)
// -----------------------------------------------------------------------------
// THE TWO LAWS OF BORROWING:
// 1. You can have EITHER one mutable reference (`&mut T`) OR any number of 
//    immutable references (`&T`) at a time.
// 2. References MUST always be valid (no dangling references).
//
// Non-Lexical Lifetimes (NLL):
// The compiler tracks reference lifetimes based on where they are LAST USED,
// rather than strictly waiting for scope end brackets `}`.
// =============================================================================

fn mutable_references_3() {
    println!("=== 4. BORROWING LIMITATIONS (CONCEPTUAL) ===");
    let s = String::from("hello");

    let r1 = &s; // Valid: Immutable borrow 1
    let r2 = &s; // Valid: Immutable borrow 2

    // Overlapping `&mut` with active `&` is forbidden:
    // let r3 = &mut s; // BIG PROBLEM: Compiler error if r1/r2 are used below!

    println!("{r1} and {r2}"); // Last use of r1 and r2
    println!("\n");
}

fn mutable_references_4() {
    println!("=== 5. NON-LEXICAL LIFETIMES (NLL) DEMO ===");
    let mut s = String::from("hello");

    let r1 = &s; // Immutable borrow 1
    let r2 = &s; // Immutable borrow 2
    println!("{r1}, {r2}"); 
    // --- LIFETIME END: r1 and r2 are never used after this line ---

    let r3 = &mut s; // OK! The compiler sees r1 and r2 are no longer active.
    println!("{r3}");
    println!("\n");
}


// =============================================================================
// SECTION 4: MEMORY DIAGRAM & FULL LIFETIME STEP-BY-STEP
// -----------------------------------------------------------------------------
// STACK                                           HEAP
// [ s: (ptr: 0x500, len: 5, cap: 5) ] ----------> [ 'h', 'e', 'l', 'l', 'o' ] (at 0x500)
//        ^
//        | (stores memory address of s)
// [ r1: 0x100 ]
//
// - Borrowing does NOT move ownership (`s` remains sole owner).
// - When reference `r1` dies: Only the small pointer on the stack disappears.
// - When owner `s` dies: Rust deallocates heap data.
// =============================================================================

fn final_mutable_references() {
    println!("=== 6. STEP-BY-STEP BORROWING MASTERCLASS ===");

    // STEP 1: Ownership & Allocation
    let mut s = String::from("hello"); 

    // STEP 2: Immutable Borrowing (&)
    let r1 = &s; 
    let r2 = &s;
    println!("Step 2: Read via r1='{r1}' and r2='{r2}'");
    // NLL Checkpoint: r1 and r2 are done. Read-locks released!

    // STEP 3: Mutable Borrowing (&mut)
    let r3 = &mut s; 
    r3.push_str(", world");
    println!("Step 3: Mutated via r3='{r3}'");
    // NLL Checkpoint: r3 is done. Exclusive lock released!

    // STEP 4: Return to Owner
    println!("Step 4: Owner s is now='{s}'");

    // STEP 5: Explicit Block Scopes
    {
        let r4 = &mut s;
        r4.push_str("!");
        println!("Step 5: Inside inner scope='{r4}'");
    } // `r4` goes out of scope and is destroyed.

    println!("Step 5 (after block): '{s}'");
    println!("\n");
}


// =============================================================================
// SECTION 5: COMPARING BAD vs. GOOD BORROWING PATTERNS
// =============================================================================

// Why Rust Enforces Aliasing Rules:
// If Rust allowed `r1` (&s) and `r3` (&mut s) simultaneously:
// 1. `r3.push_str(...)` might reallocate the heap buffer to a new address.
// 2. `r1` would still point to the OLD freed address (Dangling Pointer / Use-After-Free).
// Rust eliminates this entire class of security bugs at compile time.

fn bad_example_explanation() {
    // BROKEN PATTERN (Demonstration only):
    // let mut s = String::from("hello");
    // let r1 = &s;     // Read-only borrow active
    // let r3 = &mut s; // ERROR: Cannot borrow as mutable while r1 is active!
    // println!("r1 is: {r1}"); // `r1` used here, keeping read-lock alive!
    // r3.push_str(" world");
}

fn good_example() {
    println!("=== 7. SAFE BORROW PATTERN ===");
    let mut s = String::from("hello");

    let r1 = &s;
    println!("r1 is: {r1}"); 
    // Last use of `r1` is above. Read-lock released immediately.

    let r3 = &mut s; // Safe: No active references exist!
    r3.push_str(" world");
    println!("r3 is: {r3}");
    println!("\n");
}


// =============================================================================
// SECTION 6: PREVENTING DANGLING REFERENCES
// -----------------------------------------------------------------------------
// Dangling Reference: A pointer/reference referencing memory that has already
// been freed or allocated to another variable.
// Rust's compiler completely forbids dangling references.
// =============================================================================

// BROKEN (Compiler Error):
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ERROR: `s` is dropped at function exit, so `&s` points to dead memory!
// }

// FIX: Move ownership of `String` out to the caller.
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership moved out; memory stays alive on caller's stack frame.
}

fn dangling_reference_demo() {
    println!("=== 8. DANGLING REFERENCE PREVENTION ===");
    let safe_string = no_dangle();
    println!("Received valid heap string: {safe_string}");
    println!("\n");
}


// =============================================================================
// MAIN ENTRY POINT
// Guaranteed to compile and execute cleanly.
// =============================================================================

fn main() {
    refernce_string();
    mutable_references_1();
    mutable_references_2();
    mutable_references_3();
    mutable_references_4();
    final_mutable_references();
    good_example();
    dangling_reference_demo();
    demo_common_borrowing_errors();
}


// =============================================================================
// SECTION 7: DEMONSTRATION OF COMMON BORROWING COMPILER ERRORS
// Below are real compiler error snippets and explanations.
// =============================================================================

fn demo_common_borrowing_errors() {
    println!("=== 9. DEMONSTRATION OF BORROWING COMPILER ERRORS ===");
    println!("All compile-error snippets are documented in source comments below.");

    // Error Case 1: Cannot mutate through an immutable reference
    // let s = String::from("hello");
    // let r = &s;
    // r.push_str("world"); 
    // ERROR: [E0596] cannot borrow `*r` as mutable, as it is behind a `&` reference

    // Error Case 2: Cannot borrow as mutable when immutable reference is active
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{}", r1); 
    // ERROR: [E0502] cannot borrow `s` as mutable because it is also borrowed as immutable

    // Error Case 3: Multiple active mutable references
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2); 
    // ERROR: [E0499] cannot borrow `s` as mutable more than once at a time
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. THE RULES OF REFERENCES
--------------------------
- References let you borrow data without taking ownership.
- Immutable References (`&T`): Read-only access. You can have UNLIMITED immutable 
  references active simultaneously.
- Mutable References (`&mut T`): Read-write access. You can have EXACTLY ONE 
  mutable reference active at a time.
- NO ALIASING + MUTABILITY: You CANNOT combine `&T` and `&mut T` over overlapping 
  lifetimes.


2. NON-LEXICAL LIFETIMES (NLL)
------------------------------
- Reference lifetimes start where created and END at their LAST USE point in code, 
  not necessarily at scope closing brackets `}`.
- This allows creating a `&mut` reference later in a function as long as prior 
  `&` references are no longer read afterward.


3. DANGLING REFERENCE GUARANTEES
--------------------------------
- Rust guarantees at compile time that no reference will outlive the data it 
  points to.
- If a function creates heap data locally, return the owned value (`String`), 
  NOT a reference (`&String`).
================================================================================
*/