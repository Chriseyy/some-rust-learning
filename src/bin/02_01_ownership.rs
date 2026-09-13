// =============================================================================
// SECTION 1: THE THREE RULES OF RUST OWNERSHIP
// -----------------------------------------------------------------------------
// Ownership is Rust's core memory management strategy. It replaces both manual
// memory management (C/C++) and automatic Garbage Collection (Java/Go/Python).
//
// THE THREE LAWS:
// 1. Each value in Rust has a variable called its OWNER.
// 2. There can only be ONE owner at a time.
// 3. When the owner goes OUT OF SCOPE, the value is automatically DROPPED (freed).
//
// STACK vs. HEAP MEMORY:
// - STACK: Fast, LIFO (Last In, First Out) memory with FIXED size known at compile
//   time (e.g., integers, booleans, fixed-size arrays).
// - HEAP: Dynamic memory for data whose size is UNKNOWN at compile time or can
//   GROW dynamically (e.g., `String`, `Vec<T>`). Stores data via pointer addresses.
// =============================================================================

fn scope_example_1() {
    println!("=== 1. STACK VARIABLE SCOPE ===");
    let x = 5; // `x` is valid starting from this line (allocated on the STACK)
    println!("x: {}", x);
} // `x` goes out of scope here. Because `x` is on the stack, it is automatically popped.

fn scope_example_2() {
    println!("=== 2. STRING SLICE SCOPE (&str) ===");
    // `y` is a `&str` (string slice literal). It is an immutable reference pointing
    // directly to read-only binary text data stored in static memory.
    let y = "Hello"; 
    println!("y: {}", y);
} // `y` goes out of scope here. No dynamic heap memory needs to be deallocated.


// =============================================================================
// SECTION 2: HEAP DATA & THE `String` TYPE
// `String` is a growable, heap-allocated data structure stored as 3 parts on the stack:
// 1. Pointer (8 bytes): points to the actual text content allocated on the heap.
// 2. Length (8 bytes): how many bytes of content are currently used.
// 3. Capacity (8 bytes): total memory allocated by the OS for future growth.
// =============================================================================

fn string_example_1() {
    println!("=== 3. MUTABLE HEAP STRING ===");
    let mut s = String::from("Hello"); // Allocates buffer on the HEAP
    s.push_str(", world!");            // Reallocates/extends memory on the heap
    println!("s: {}", s);
} // `s` goes out of scope here. Rust automatically calls `drop` to free heap memory!

fn string_example_2() {
    println!("=== 4. CREATING STRING FROM SLICE ===");
    // `.to_string()` allocates new memory on the heap to copy the `&str` contents
    let s1 = "Hello".to_string(); 
    
    // MOVE SEMANTICS: Ownership of the heap buffer is transferred from `s1` to `s2`.
    // `s1` is invalidated by the compiler to prevent double-free bugs!
    let s2 = s1; 
    
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // COMPILE ERROR: Use of moved value `s1`
} // `s2` goes out of scope here and calls `drop`. `s1` was moved, so no drop is run for `s1`.


// =============================================================================
// SECTION 3: MEMORY ALLOCATION & MOVE vs. CLONE
// Rust guarantees memory safety without a garbage collector by pairing exactly 
// ONE allocation with exactly ONE deallocation (`drop`) when a variable exits scope.
// =============================================================================

fn memory_example_1() {
    println!("=== 5. AUTOMATIC MEMORY CLEANUP (DROP) ===");
    let s = String::from("Hello, world!"); 
    println!("s: {}", s);
} // `s` leaves scope -> Rust automatically invokes `String::drop(&mut s)` to free heap data.

fn memory_example_2() {
    println!("=== 6. MOVE vs. CLONE vs. COPY ===");

    // A. STACK COPY (Types implementing the `Copy` trait)
    let x = 5;
    let y = x; // Primitive integers are stored completely on the STACK. `y` is a bitwise copy of `x`.
    println!("x: {}, y: {}", x, y); // Both `x` and `y` remain fully valid!

    // B. HEAP MOVE (Types NOT implementing the `Copy` trait)
    let s1 = String::from("hello");
    
    // DIAGRAM OF A MOVE:
    // s1 (Stack): [Pointer | Len: 5 | Cap: 5]  ---> [ h | e | l | l | o ] (Heap)
    // s2 = s1:
    // s2 (Stack): [Pointer | Len: 5 | Cap: 5]  ---> [ h | e | l | l | o ] (Heap)
    // s1 IS NOW INVALIDATED / DEAD.
    let s2 = s1; 
    println!("s2 (after move): {}", s2);
    // println!("s1: {}", s1); // ERROR! `s1` is no longer valid.

    // C. HEAP DEEP COPY (.clone())
    let s3 = String::from("world");
    let s4 = s3.clone(); // Duplicates the actual text bytes on the heap. Both `s3` and `s4` are valid.
    println!("s3: {}, s4 (cloned): {}", s3, s4);
    println!("\n");
}


// =============================================================================
// SECTION 4: SCOPE REASSIGNMENTS & COPY TRAIT DETAILS
// =============================================================================

fn scope_example_3() {
    let x = 5;
    println!("x: {}", x);
}

fn scope_example_4() {
    println!("=== 7. REASSIGNING MUTABLE HEAP STRINGS ===");
    let mut y = String::from("Hello"); 
    println!("y (initial): {}", y);

    // Reassigning `y`: The old heap buffer ("Hello") is IMMEDIATELY dropped and freed.
    // A brand-new heap buffer ("World") is allocated and assigned to `y`.
    y = String::from("World"); 
    println!("y (reassigned): {}", y);
    println!("\n");
}

fn deeply_copy_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Explicit deep copy (may execute expensive heap allocations)
    println!("s1 = {s1}, s2 = {s2}");
}

fn stack_only_copy_example() {
    let x = 5;
    let y = x; // Copying stack-only data is trivial and instant
    println!("x = {x}, y = {y}");
}

fn important_copys() {
    println!("=== 8. SUMMARY OF COPY, MOVE, AND DEEP CLONE ===");
    
    // 1. Simple Stack Copy (Trivial byte copy on stack; no heap involved)
    let x = 5;
    let y = x; 
    println!("Stack Copy y: {}", y);

    // 2. Shallow Copy / MOVE (Only pointer/metadata is copied; original variable invalidated)
    let s1 = String::from("Hello");
    let s2 = s1; 
    println!("Moved String s2: {}", s2);

    // 3. DEEP COPY (Heap data is duplicated; both variables independent)
    let s3 = String::from("World");
    let s4 = s3.clone();
    println!("Cloned String s4: {}", s4);
    println!("\n");
}
// Which types implement the `Copy` trait?
// - All integer types (u8, i32, usize, etc.)
// - All floating-point types (f32, f64)
// - The boolean type (bool)
// - The character type (char)
// - Tuples, IF all contained fields implement `Copy` (e.g., `(i32, i32)` implements `Copy`, but `(i32, String)` does NOT).


// =============================================================================
// SECTION 5: OWNERSHIP AND FUNCTIONS
// Passing a variable into a function follows the exact same rules as assignment:
// - Passing a heap value (like `String`) MOVES ownership into the function parameter.
// - Passing a stack value (like `i32`) COPIES the value into the function parameter.
// =============================================================================

fn ownership_example() {
    println!("=== 9. OWNERSHIP TRANSFER IN FUNCTIONS ===");

    let s = String::from("hello"); // `s` comes into scope

    takes_ownership(s);            // `s` is MOVED into `takes_ownership`.
                                   // `s` is NO LONGER VALID in this scope!
    
    // println!("{}", s);          // ERROR! Compile time safety check blocks this.
    // Fix: `takes_ownership(s.clone());` leaves `s` valid here.

    let x = 5;                     // `x` comes into scope

    makes_copy(x);                 // `i32` implements `Copy`, so `x` is COPIED into `makes_copy`.
    println!("x after function call: {}", x); // `x` remains valid here!
    println!("\n");
}

fn takes_ownership(some_string: String) { // `some_string` comes into scope and takes ownership
    println!("takes_ownership received: {some_string}");
} // `some_string` goes out of scope here -> `drop` is called, and heap memory is freed!

fn makes_copy(some_integer: i32) { // `some_integer` comes into scope as a copy
    println!("makes_copy received: {some_integer}");
} // `some_integer` goes out of scope. Nothing special happens.


// =============================================================================
// SECTION 6: RETURN VALUES & TRANSFERRING SCOPE OWNERSHIP
// Returning values from functions also transfers ownership back to the caller!
// =============================================================================

fn return_scope_example_1() {
    println!("=== 10. RETURNING OWNERSHIP FROM FUNCTIONS ===");

    let s1 = gives_ownership();         // `gives_ownership` moves its return value into `s1`

    let s2 = String::from("hello");     // `s2` comes into scope

    let s3 = takes_and_gives_back(s2);  // `s2` moves into `takes_and_gives_back`,
                                        // which returns its value and moves ownership into `s3`
    
    println!("s1: {}", s1);
    println!("s3: {}", s3);
    // println!("s2: {}", s2);          // ERROR! `s2` was moved into `takes_and_gives_back`.
    println!("\n");
} // `s3` is dropped. `s2` was moved (nothing happens). `s1` is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // `some_string` comes into scope
    some_string                              // `some_string` is returned and moves ownership out to caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // `a_string` is returned and moves ownership out to caller
}

fn return_scope_example_2() {
    println!("=== 11. RETURNING MULTIPLE VALUES VIA TUPLES ===");
    let s1 = String::from("hello");

    // Without borrowing, to keep using `s1`, the function must return ownership along with the result!
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
    println!("\n");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // `.len()` returns length in bytes
    (s, length)           // Return string ownership along with computed length
}


// =============================================================================
// MAIN ENTRY POINT
// Guaranteed to compile and run cleanly.
// =============================================================================

fn main() {
    println!("=== RUST OWNERSHIP & SCOPE MASTERCLASS ===\n");

    println!("Scope Examples:");
    scope_example_1();
    scope_example_2();

    println!("String Type Examples:");
    string_example_1();
    string_example_2();

    println!("Memory and Allocation Examples:");
    memory_example_1();
    memory_example_2();

    println!("Scope and Assignment Examples:");
    scope_example_3();
    scope_example_4();

    println!("Data Interaction (Clone & Copy):");
    deeply_copy_example();
    stack_only_copy_example();
    important_copys();

    println!("Ownership and Functions:");
    ownership_example();

    println!("Return Values and Scope:");
    return_scope_example_1();
    return_scope_example_2();

    demo_common_ownership_errors();
}


// =============================================================================
// SECTION 7: DEMONSTRATION OF COMMON OWNERSHIP ERRORS
// Below are real compiler error cases and explanations of why Rust rejects them.
// =============================================================================

fn demo_common_ownership_errors() {
    println!("=== 12. DEMONSTRATION OF OWNERSHIP COMPILER ERRORS ===");
    println!("All compile-error snippets are documented in source comments below.");

    // Error Case 1: Use of Moved Value
    // let s1 = String::from("Rust");
    // let s2 = s1;
    // println!("{}", s1); 
    // ERROR: [E0382] borrow of moved value: `s1`

    // Error Case 2: Use after moving into function
    // let s = String::from("Rust");
    // takes_ownership(s);
    // println!("{}", s); 
    // ERROR: [E0382] borrow of moved value: `s`

    // Error Case 3: Tuple containing non-Copy type cannot be copied
    // let t1 = (5, String::from("Rust"));
    // let t2 = t1;
    // println!("{:?}", t1); 
    // ERROR: [E0382] borrow of moved value: `t1` (because String inside tuple doesn't implement Copy)
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. THE THREE OWNERSHIP RULES
----------------------------
- Every value has an owner variable.
- Only one owner exists at a time.
- When the owner goes out of scope, Rust automatically frees the value (`drop`).


2. STACK vs. HEAP BEHAVIOR
--------------------------
- STACK: Known, fixed size at compile time. Data is copied trivially (`Copy` trait).
- HEAP: Dynamic size allocated at runtime. Data is MOVED by default when assigned 
  or passed to functions.


3. MOVE vs. CLONE vs. BORROWING
-------------------------------
- MOVE (Default for Heap): Transmits ownership of heap pointer/metadata. Fast and 
  prevents double-free bugs by invalidating the source variable.
- CLONE (`.clone()`): Performs an explicit deep copy of heap contents. Expensive 
  in memory/time, but leaves both original and new variables valid.
- BORROWING (`&T` / `&mut T`): Allows inspecting or mutating data WITHOUT taking 
  ownership (covered in References & Borrowing).
================================================================================
*/