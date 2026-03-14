

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

// :: String is a growable, heap-allocated data structure that is used to store and manipulate text. It is part of the Rust standard library and provides a convenient way to work with strings in Rust.

fn string_example_1() {
    let mut s = String::from("Hello"); // s is valid from this point
    s.push_str(", world!"); // s can be modified because it is mutable
    println!("s: {}", s);
} // s goes out of scope here, and its memory is deallocated


// .to_string() is a method that can be called on string literals to create a String instance. It is a convenient way to convert a string literal into a String type.
// without .to_string(), the string literal is of type &str, which is a string slice. It is a reference to a string that is stored on the stack and has a known size at compile time.
fn string_example_2() {
    let s1 = "Hello".to_string(); // s1 is valid from this point
    let s2 = s1; // s2 takes ownership of the string, and s1 is no longer valid
    println!("s2: {}", s2);
} // s2 goes out of scope here, and its memory is deallocated



// Memory and Allocation
// with String type in order to store a string on the heap, Rust needs to allocate memory for it. 
// When a String is created, Rust allocates memory on the heap to store the string data. The String type manages this memory automatically, so you don't have to worry about it. 
// When a String goes out of scope, Rust will automatically deallocate the memory that was allocated for it on the heap. This is one of the key features of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.

fn memory_example_1() {
    let s = String::from("Hello, world!"); // s is valid from this point
    println!("s: {}", s);

    // do something with s
} // s goes out of scope here, and its memory is deallocated
// rust calls drop function to deallocate the memory that was allocated for s on the heap. This is done automatically by Rust's ownership system, so you don't have to worry about it.


// rust does not have a garbage collector, so it relies on its ownership system to manage memory. When a value goes out of scope, Rust automatically deallocates the memory that was allocated for it. This is a key feature of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.
// We need to pair exactly one allocate with exactly one free. The memory is automatically returned once the variable that owns it goes out of scope. 


// Variables and Data Interacting with Move

// ssigning the integer value of variable x to y
fn memory_example_2() {
    let x = 5;
    let y = x; // y is a copy of x, and x is still valid
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // s2 takes ownership of the string, and s1 is no longer valid  / this is shallow copy casue s1 wont work after 
    // if you dont want ownership to be moved, you can use the clone method to create a copy of the string data on the heap. This will allow both s1 and s2 to be valid and independent of each other.
    // let s2 = s1.clone(); // s2 is a clone of s1, and both s1 and s2 are valid / this is deep copy cause s1 still workds after 
    println!("s2: {}", s2);
    // println!("s1: {}, s2: {}", s1, s2); // this will cause a compile-time error because s1 is no longer valid after it is moved to s2        
} 
// In this example, x is an integer value that is stored on the stack. When we assign x to y, Rust creates a copy of the value of x and assigns it to y. 
// Both x and y are valid and can be used independently. This is because integers are a simple data type that implements the Copy trait, which means that they can be copied rather than moved.
// On the other hand, s1 is a String value that is stored on the heap. When we assign s1 to s2, Rust moves the ownership of the string from s1 to s2.
// After this assignment, s1 is no longer valid and cannot be used. This is because String is a complex data type that does not implement the Copy trait, which means that it cannot be copied and must be moved instead.

// string under the covers:
// s1 name    |  pointer to the heap data  |  length of the string  |  capacity of the string
// pointer to the heap data is a reference to the location in memory where the string data is stored.
// s2 name    |  pointer to the heap data  |  length of the string  |  capacity of the string
// When we assign s1 to s2, Rust moves the ownership of the string from s1 to s2. This means that s2 now has the pointer to the heap data, the length of the string, and the capacity of the string. s1 is no longer valid and cannot be used, because it no longer has ownership of the string data.
// doesnt copy the string data on the heap, it just moves the ownership of the pointer to the heap data from s1 to s2. This is why s1 is no longer valid after the assignment, because it no longer has ownership of the string data on the heap.
// s1 cant be used after the assignment because it no longer has ownership of the string data on the heap. If we try to use s1 after the assignment, we will get a compile-time error because s1 is no longer valid. This is one of the key features of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.

// when copy and when move happens:
// Copy happens when the type of the value implements the Copy trait. This includes simple data types like integers, floating-point numbers, booleans, and characters. When a value of a type that implements the Copy trait is assigned to another variable, Rust creates a copy of the value and assigns it to the new variable. 
// Both variables are valid and can be used independently.
// all what is know at compile time can be copied, because it has a known size and can be stored on the stack. This is why simple data types like integers and booleans implement the Copy trait, while complex data types like String and Vec do not implement the Copy trait and instead require ownership to be moved.

// Move happens when the type of the value does not implement the Copy trait. This includes complex data types like String, Vec, and HashMap. When a value of a type that does not implement the Copy trait is assigned to another variable, Rust moves the ownership of the value from the original variable to the new variable.
// After the move, the original variable is no longer valid and cannot be used. This is because the ownership of the value has been transferred to the new variable, and the original variable no longer has access to the value. This is one of the key features of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.
// all what is not known at compile time must be moved, because it has a dynamic size and cannot be stored on the stack. This is why complex data types like String and Vec do not implement the Copy trait and instead require ownership to be moved. all thats is dynamically sized must be moved, because it cannot be stored on the stack and must be stored on the heap. This is why complex data types like String and Vec do not implement the Copy trait and instead require ownership to be moved.


// Scope and Assignments:
// When a variable goes out of scope, Rust automatically deallocates the memory that was allocated for it. This is one of the key features of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.
// When a variable is assigned to another variable, Rust either creates a copy of the value (if the type implements the Copy trait) or moves the ownership of the value (if the type does not implement the Copy trait). This is one of the key features of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.

fn scope_example_3() {
    let x = 5; // x is valid from this point
    println!("x: {}", x);
} // x goes out of scope here, and its memory is deallocated

fn scope_example_4() {
    let mut y = String::from("Hello"); // y is valid from this point
    println!("y: {}", y);
    y = String::from("World"); // y can be modified because it is mutable
    // "Hello" is deallocated from the heap, and "World" is allocated on the heap and assigned to y "Hello" is dropped and its memory is deallocated when y is reassigned to "World". This is because the ownership of the string data on the heap is transferred from "Hello" to "World", and "Hello" is no longer valid and cannot be used. This is one of the key features of Rust's ownership system, as it helps to prevent memory leaks and ensures that resources are properly managed.
    println!("y: {}", y);
} // y goes out of scope here, and its memory is deallocated


// Variables and Data interaction with Clone:
// if we do want to deeply copy the the heap data you can use the clone Methode

fn deeply_copy_example()  {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
// clone: aribitary code is being executed and that code may be expensive 


// Stack-Only Data: Copy 
fn stack_only_copy_example() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}
// no clone needed cause  integers that have a known size at compile time are stored entirely on the stack
// Simple stack copy 

fn important_copys() {
    // 1. Simple Stack Copy (Not shallow or deep, just a basic copy)
    let x = 5;
    let y = x; 
    println!("{}",y);

    // 2. Shallow Copy / MOVE (Only pointer is copied, s1 is killed)
    let s1 = String::from("Hello");
    let s2 = s1; 
    println!("{}",s2);

    // 3. DEEP COPY (Heap data is duplicated, both s3 and s4 are valid and independent)
    let s3 = String::from("World");
    let s4 = s3.clone();

    println!("{}",s4);
}

// Copy trait: 
// Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). 
// If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
// So, what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, 
// and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

// All the integer types, such as u32.
// The Boolean type, bool, with values true and false.
// All the floating-point types, such as f64.
// The character type, char.
// Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.



// Ownership and Functions

fn ownership_example() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // println!("{}",s);  // wont work anymore case ownership is now long - error 
    // fix would be takes_ownership(s.clone()); 

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.
    println!("{}",x);               // works still cause it makes copy of x and donest steal ownership                        

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn main() {
    println!("Scope Examples:");
    scope_example_1();
    scope_example_2();
    
    println!("------------------------------");

    println!("String Type:");
    string_example_1();
    string_example_2();

    println!("------------------------------");
    println!("Memory and Allocation:");
    memory_example_1();
    memory_example_2();

    println!("Scope and Assignments:");
    scope_example_3();
    scope_example_4();

    println!("Variables and Data interaction with Clone:");
    deeply_copy_example();
    stack_only_copy_example();
    important_copys();

    println!("------------------------------");
    println!("Ownership and Function:");
    ownership_example();
}