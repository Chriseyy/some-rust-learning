

// Ownership
// Ownership is a fundamental concept in Rust that governs how memory is managed. It is based on three rules:
// 1. Each value in Rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.  


// First stack and heap:
// stack is a region of memory that is used for storing values that have a known size at compile time. It is fast and efficient, but it has limited capacity. When a value is stored on the stack, it is automatically deallocated when it goes out of scope.

// heap is a region of memory that is used for storing values that have a dynamic size or that are too large to fit on the stack. It is slower than the stack, but it has much larger capacity. 
// When a value is stored on the heap, it must be manually allocated and deallocated using the Box type or other smart pointer types.  




// Variable Scope:
// Scope is a region of the program where a variable is valid. In Rust, variables are valid from the point they are declared until the end of the block they are declared in. 
// When a variable goes out of scope, it is automatically dropped and its memory is deallocated. This is an important aspect of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.

fn scope_example_1() {
    let x = 5; // x is valid from this point
    println!("x: {}", x);
} // x goes out of scope here, and its memory is deallocated


fn scope_example_2() {
    // let y = String::from("Hello"); // y is valid from this point
    let y = "Hello"; // &str is a string slice, which is a reference to a string. It is stored on the stack and has a known size at compile time.
    println!("y: {}", y);
} // y goes out of scope here, and its memory is deallocated

// In other words, there are two important points in time here:
// When s comes into scope, it is valid.
// It remains valid until it goes out of scope.




// String Type:


fn main() {
    println!("Scope Examples:");
    scope_example_1();
    scope_example_2();
    
    println!("------------------------------");

    println!("String Type:");
}