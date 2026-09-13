// =============================================================================
// SECTION 1: METHOD SYNTAX & THE `impl` BLOCK
// -----------------------------------------------------------------------------
// Methods are functions defined within the context of a struct, enum, or trait object.
// - Declared inside an `impl` (implementation) block.
// - Their first parameter is always `self`, which represents the instance the method is called on.
// - `&self` is short for `self: &Self`, where `Self` is an alias for the type inside `impl`.
//
// Variants of Receiver Types:
// 1. `&self`: Borrows the instance immutably (most common; read-only access).
// 2. `&mut self`: Borrows the instance mutably (allows changing struct fields).
// 3. `self`: Takes ownership of the instance (consumes it; rare, used for transformations).
// =============================================================================

struct Rectangle {
    width: u32,
    height: u32,
}

// First implementation block for `Rectangle`
impl Rectangle {
    // Calculates the area of a rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Rust allows splitting methods into multiple `impl` blocks for the same type!
impl Rectangle {
    // Getter-like method matching field name: checks if width is non-zero
    fn width(&self) -> bool {
        self.width > 0
    }
}


// =============================================================================
// SECTION 2: AUTOMATIC REFERENCING AND DEREFERENCING
// -----------------------------------------------------------------------------
// In languages like C/C++, you use `.` for objects and `->` for pointers (`p->distance()`).
// Rust does NOT have a `->` operator. Instead, Rust uses AUTOMATIC REFERENCING:
//
// When you call `object.something()`, Rust automatically adds `&`, `&mut`, or `*`
// so that `object` matches the method signature!
//
// Equivalences:
// `p1.distance(&p2)`  ==  `(&p1).distance(&p2)`
// =============================================================================


// =============================================================================
// SECTION 3: METHODS WITH MULTIPLE PARAMETERS
// -----------------------------------------------------------------------------
// Methods can accept additional parameters alongside `&self`.
// =============================================================================

fn methonds_more_parameters() {
    println!("=== 1. METHODS WITH ADDITIONAL PARAMETERS ===");

    struct LocalRectangle {
        width: u32,
        height: u32,
    }

    impl LocalRectangle {
        // Checks if `self` can completely fit `other` inside its bounds
        fn can_hold(&self, other: &LocalRectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = LocalRectangle {
        width: 30,
        height: 50,
    };
    let rect2 = LocalRectangle {
        width: 10,
        height: 40,
    };
    let rect3 = LocalRectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true (30 > 10 && 50 > 40)
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false (30 < 60)
    println!("\n");
}


// =============================================================================
// SECTION 4: ASSOCIATED FUNCTIONS (CONSTRUCTORS)
// -----------------------------------------------------------------------------
// All functions inside an `impl` block are "associated functions".
// However, functions that DO NOT take `self` as a parameter are NOT methods.
//
// Characteristics:
// - They don't need an instance to be called.
// - Frequently used as constructors (e.g., `String::from()`, `Rectangle::square()`).
// - Called using namespace syntax `Type::function_name()` instead of dot notation `.`.
// =============================================================================

fn associated_functions() {
    println!("=== 2. ASSOCIATED FUNCTIONS (CONSTRUCTORS) ===");

    struct LocalRectangle {
        width: u32,
        height: u32,
    }

    impl LocalRectangle {
        // Associated function that creates and returns a square instance
        fn square(size: u32) -> LocalRectangle {
            LocalRectangle {
                width: size,
                height: size,
            }
        }
    }

    // Called via `Type::function_name(args)` syntax
    let sq = LocalRectangle::square(3);
    println!("Square dimensions: {} x {}", sq.width, sq.height);
    println!("\n");
}


// =============================================================================
// SECTION 5: MULTIPLE `impl` BLOCKS
// -----------------------------------------------------------------------------
// Rust allows dividing methods across multiple `impl` blocks.
// Useful when organizing large codebases or grouping traits/generics.
// =============================================================================

fn multi_impl_blocks() {
    println!("=== 3. MULTIPLE IMPL BLOCKS DEMO ===");

    struct LocalRectangle {
        width: u32,
        height: u32,
    }

    impl LocalRectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl LocalRectangle {
        fn can_hold(&self, other: &LocalRectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = LocalRectangle {
        width: 30,
        height: 50,
    };
    let rect2 = LocalRectangle {
        width: 10,
        height: 40,
    };
    let rect3 = LocalRectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Rect1 Area: {}", rect1.area());
    println!("\n");
}


// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

fn main() {
    println!("=== RUST METHODS & IMPL BLOCKS MASTERCLASS ===\n");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // Rust automatically borrows `&rect1` to pass into `area(&self)`
    );

    if rect1.width() {
        println!("The rectangle has a non-zero width of {} pixels.", rect1.width);
    }
    println!("\n");

    methonds_more_parameters();
    associated_functions();
    multi_impl_blocks();

    demo_common_method_errors();
}


// =============================================================================
// SECTION 6: DEMONSTRATION OF COMMON METHOD COMPILER ERRORS
// Below are real compile-error cases and explanations.
// =============================================================================

fn demo_common_method_errors() {
    println!("=== 4. DEMONSTRATION OF METHOD COMPILER ERRORS ===");
    println!("All compile-error snippets are documented in source comments below.");

    // Error Case 1: Trying to call a method that mutates fields via `&self`
    // struct Counter { count: u32 }
    // impl Counter {
    //     fn increment(&self) { self.count += 1; } // ERROR: cannot mutate field in an immutable borrow
    // }

    // Error Case 2: Calling an associated function with dot notation `. square()`
    // let rect = Rectangle { width: 10, height: 10 };
    // rect.square(5); 
    // ERROR: no method named `square` found for struct `Rectangle` (must call `Rectangle::square(5)`)

    // Error Case 3: Calling a method taking `&mut self` on an immutable instance
    // struct Device { state: bool }
    // impl Device { fn turn_on(&mut self) { self.state = true; } }
    // let dev = Device { state: false }; // Not declared `mut`
    // dev.turn_on(); 
    // ERROR: cannot borrow `dev` as mutable, as it is not declared as mutable
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. METHODS VS. ASSOCIATED FUNCTIONS
-----------------------------------
- Methods: Have `self`, `&self`, or `&mut self` as their first parameter. 
  Called on instances using dot syntax (`instance.method()`).
- Associated Functions: Do NOT take a `self` parameter. 
  Called on types using namespace syntax (`Type::function()`).


2. CHOOSING THE RIGHT RECEIVER TYPE
-----------------------------------
- `&self`: Use when reading fields without changing state (most common).
- `&mut self`: Use when updating internal fields or state.
- `self`: Use when consuming/destroying the caller (e.g., converting types or building pipelines).


3. AUTOMATIC REFERENCING
------------------------
- Rust automatically dereferences or references method callers (`rect1.area()` 
  automatically becomes `(&rect1).area()`).
- Eliminates the need for explicit pointer dereferencing arrows (`->`) seen in C/C++.


4. MULTIPLE `impl` BLOCKS
-------------------------
- Valid in Rust to separate methods into multiple `impl` blocks.
- Useful for keeping code clean, organizing conditional compilation, or 
  implementing traits separately.
================================================================================
*/