// =============================================================================
// SECTION 1: WHAT IS A STRUCT IN RUST?
// -----------------------------------------------------------------------------
// A struct (structure) is a custom data type that lets you package together
// multiple related values into a single meaningful group.
//
// Types of Structs in Rust:
// 1. Classic Named-Field Structs: Named fields with specific types.
// 2. Tuple Structs: Named type with ordered, unnamed fields.
// 3. Unit-Like Structs: Fieldless types useful for trait implementations.
// =============================================================================

fn example_struct() {
    println!("=== 1. BASIC NAMED STRUCT DEMO ===");

    // Local struct definition
    struct Person {
        name: String,
        age: u32, // `u32` ensures age is non-negative
    }

    // Instantiating a `Person` struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Accessing fields using dot notation (`person1.field`)
    println!("Name: {}, Age: {}", person1.name, person1.age);
    println!("\n");
}


// =============================================================================
// SECTION 2: FIELD INIT SHORTHAND & STRUCT INSTANTIATION
// -----------------------------------------------------------------------------
// Global/Module-level struct definition for `User`.
// Field Init Shorthand: If parameter names match struct field names exactly,
// you can write `email` instead of `email: email`.
// =============================================================================

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // Shorthand for `email: email`
        username, // Shorthand for `username: username`
        active: true,
        sign_in_count: 1,
    }
}


// =============================================================================
// SECTION 3: STRUCT UPDATE SYNTAX (`..`) & OWNERSHIP TRANSFER
// -----------------------------------------------------------------------------
// The struct update syntax (`..user1`) populates unassigned fields from an existing instance.
//
// OWNERSHIP WARNING:
// - If fields being updated implement `Copy` (e.g., `bool`, `u64`), they are copied.
// - If fields DO NOT implement `Copy` (e.g., `String`), ownership is MOVED!
// - Using `..user1` will move `user1.email` into `user3`, making `user1` partially invalid.
// =============================================================================

fn struct_update_example() {
    println!("=== 2. STRUCT UPDATE SYNTAX & OWNERSHIP ===");

    let user1 = build_user(String::from("mutable@fdsafg.com"), String::from("mutable"));

    // Manual copy avoiding move by cloning heap data (`String`) explicitly
    let user2 = User {
        email: user1.email.clone(), // Cloned so `user1.email` remains valid
        username: String::from("user2"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!(
        "User1: {}, Email: {}, Active: {}, Sign-in Count: {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );
    println!(
        "User2: {}, Email: {}, Active: {}, Sign-in Count: {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    // Using Struct Update Syntax (`..user1`)
    // NOTE: This MOVES `user1.email` into `user3` because `username` was overridden, 
    // but `email` was NOT overridden and is not a `Copy` type!
    let user3 = User {
        username: String::from("user3"),
        ..user1 // Moves `user1.email`, copies `user1.active` and `user1.sign_in_count`
    };

    println!(
        "User3: {}, Email: {}, Active: {}, Sign-in Count: {}",
        user3.username, user3.email, user3.active, user3.sign_in_count
    );

    // `user1.username` can still be printed because it wasn't moved, but `user1.email` CANNOT!
    // println!("{}", user1.email); // ERROR: Use of moved value `user1.email`
    println!("\n");
}


// =============================================================================
// SECTION 4: TUPLE STRUCTS
// -----------------------------------------------------------------------------
// Tuple Structs have a struct name, but their fields have no names—only types.
// Useful when giving a whole tuple a distinct type name without naming every field.
// =============================================================================

fn types_with_tuple_structs() {
    println!("=== 3. TUPLE STRUCTS DEMO ===");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access elements via tuple dot-indexing: `.0`, `.1`, `.2`
    println!("Color RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point XYZ: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Type Safety Guarantee:
    // Even though `Color` and `Point` are both `(i32, i32, i32)`, Rust treats them
    // as completely distinct types! A function expecting `Color` will reject a `Point`.
    println!("\n");
}


// =============================================================================
// SECTION 5: UNIT-LIKE STRUCTS (NO FIELDS)
// -----------------------------------------------------------------------------
// Unit-like structs have no fields at all (resembling `()`, the unit tuple).
// Best used when you need to implement a Trait on a type, but don't need to store state.
// =============================================================================

fn empty_structs() {
    println!("=== 4. UNIT-LIKE STRUCTS DEMO ===");

    struct AlwaysEqual; // Declared without braces or parentheses

    let _subject = AlwaysEqual; // Instantiated without braces or parentheses
    
    println!("Successfully created an instance of Unit-Like Struct 'AlwaysEqual'!");
    println!("\n");
}


// =============================================================================
// SECTION 6: STRUCTS AND LIFETIMES (&str vs. String)
// -----------------------------------------------------------------------------
// Can a struct hold references like `&str` instead of owned `String`?
// YES, but it requires explicit LIFETIME PARAMETERS (`'a`).
// Without lifetimes, Rust cannot guarantee the struct outlives the borrowed string!
// =============================================================================

fn ownership_of_structs_explanation() {
    // BROKEN STRUCT DEFINITION (without lifetime parameters):
    // struct BorrowedUser {
    //     active: bool,
    //     username: &str, // ERROR: Missing lifetime specifier!
    //     email: &str,    // ERROR: Missing lifetime specifier!
    //     sign_in_count: u64,
    // }

    // CORRECT WAY USING LIFETIMES:
    #[allow(dead_code)]
    struct BorrowedUser<'a> {
        active: bool,
        username: &'a str, // `'a` ensures struct instance cannot outlive referenced string
        email: &'a str,
        sign_in_count: u64,
    }

    let user = BorrowedUser {
        active: true,
        username: "someusername123", // Static string literal slice (&'static str)
        email: "some@email.com",
        sign_in_count: 1,
    };

    println!("Borrowed user username: {}", user.username);
}


// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

fn main() {
    println!("=== RUST STRUCTS MASTERCLASS ===\n");

    example_struct();

    let email = String::from("heheh@fdsafg.com");
    let username = String::from("heheh");
    let user = build_user(email, username);
    println!(
        "User: {}, Email: {}, Active: {}, Sign-in Count: {}",
        user.username, user.email, user.active, user.sign_in_count
    );

    // MUTABILITY RULE IN RUST:
    // Entire struct instances are either mutable OR immutable.
    // Rust DOES NOT allow marking individual fields as mutable (e.g., `struct User { mut email: String }` is invalid).
    let mut mutable_user = build_user(String::from("mutable@fdsafg.com"), String::from("mutable"));
    println!(
        "Mutable User Initial Email: {}",
        mutable_user.email
    );

    // Mutating a field in a `mut` struct:
    mutable_user.email = String::from("mutable_user@email.com");
    println!(
        "Updated Mutable User Email: {}",
        mutable_user.email
    );
    println!("\n");

    struct_update_example();
    types_with_tuple_structs();
    empty_structs();
    ownership_of_structs_explanation();

    demo_common_struct_errors();
}


// =============================================================================
// SECTION 7: DEMONSTRATION OF COMMON STRUCT COMPILER ERRORS
// Below are real compile-error cases and explanations.
// =============================================================================

fn demo_common_struct_errors() {
    println!("=== 5. DEMONSTRATION OF STRUCT COMPILER ERRORS ===");
    println!("All compile-error snippets are documented in source comments below.");

    // Error Case 1: Attempting to mutate a field on an immutable struct instance
    // let user = build_user(String::from("a@b.com"), String::from("user"));
    // user.email = String::from("new@b.com"); 
    // ERROR: [E0594] cannot assign to `user.email`, as `user` is not declared as mutable

    // Error Case 2: Partial move of non-Copy field via Struct Update Syntax
    // let user1 = build_user(String::from("a@b.com"), String::from("u1"));
    // let user2 = User { username: String::from("u2"), ..user1 };
    // println!("{}", user1.email); 
    // ERROR: [E0382] borrow of moved value: `user1.email`

    // Error Case 3: Storing references without lifetime annotations
    // struct RefHolder { data: &str } 
    // ERROR: [E0106] missing lifetime specifier
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. THREE TYPES OF STRUCTS
-------------------------
- Classic Named Structs (`struct Person { name: String }`): Clear field names.
- Tuple Structs (`struct Point(i32, i32)`): Compact, type-safe tuples.
- Unit-Like Structs (`struct AlwaysEqual;`): Zero size, useful for implementing traits.


2. STRUCT MUTABILITY IN RUST
----------------------------
- Mutability is an ALL-OR-NOTHING property of the variable binding (`let mut user = ...`).
- You cannot mark individual fields inside a struct as `mut`.


3. STRUCT UPDATE SYNTAX (`..`) & OWNERSHIP
------------------------------------------
- `..user1` copies fields that implement `Copy` and MOVES non-`Copy` fields (`String`, `Vec`).
- If a non-`Copy` field moves into the new struct, the original struct instance can no 
  longer be used as a whole.


4. REFERENCES IN STRUCTS
------------------------
- Holding owned data (`String`) inside structs is recommended for beginners.
- Holding references (`&str`) requires lifetime annotations (e.g., `struct User<'a>`).
================================================================================
*/