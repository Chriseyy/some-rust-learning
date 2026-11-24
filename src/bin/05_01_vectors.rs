// =============================================================================
// VECTORS (`Vec<T>`) - CHAPTER 8.1
// =============================================================================

fn new_vector() {
    let _v: Vec<i32> = Vec::new(); // Empty vector with explicit type annotation
    let _v = vec![1, 2, 3];        // Macro infers Vec<i32>

    let mut v = Vec::new();
    v.push(4); // Type is inferred as Vec<i32> from the first pushed value
    v.push(5);

    // v.push("Hello"); // ERROR: A vector can only store values of the same type
}

fn enum_vec() {
    // Idiomatic way to store multiple types in a single Vec: use an enum
    enum MyType {
        Number(i32),
        Text(String),
    }

    let mut my_values: Vec<MyType> = Vec::new();
    my_values.push(MyType::Number(42));
    my_values.push(MyType::Text("Hello".to_string()));

    for value in my_values {
        match value {
            MyType::Number(n) => println!("Number: {n}"),
            MyType::Text(s) => println!("Text: {s}"),
        }
    }

    // Alternative (rarely recommended): Box<dyn Any> allows arbitrary types,
    // but you lose compile-time type safety and have to downcast at runtime.
    use std::any::Any;
    let mut anything: Vec<Box<dyn Any>> = Vec::new();
    anything.push(Box::new(42));
    anything.push(Box::new("Text"));
}

fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    // Borrowing an element via index (panics if out of bounds)
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Copying an element out: completely type-safe here because `i32` implements `Copy`.
    // (For non-Copy types like `String`, `let s = v[2];` would fail to compile.)
    let third_copy: i32 = v[2];
    println!("Copied third element: {third_copy}");

    // Safe access with `.get()` (returns Option<&T>, never panics)
    match v.get(2) {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // PANICS at runtime!
    let _does_not_exist = v.get(100); // Safely returns None

    v.iter().for_each(|x| println!("{x}"));
    println!("Vec pos 3 is {}", v[3]);
}

// =============================================================================
// CLASSIC BORROW CHECKER TRAP WITH VECTORS
// =============================================================================
fn borrow_checker_trap() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // Immutable borrow of `v` starts here

    // v.push(6);
    // COMPILE ERROR [E0502]: cannot borrow `v` as mutable because it is also
    // borrowed as immutable.
    // WHY? Pushing a new element might require allocating new memory on the heap
    // and moving the entire vector, which would leave `first` pointing to freed memory!

    println!("The first element is: {first}"); // Immutable borrow ends here

    v.push(6); // Now valid! `first` is no longer used after this point (NLL).
    println!("After push: {:?}", v);
}

fn iterating_over_vector() {
    let v = vec![1, 2, 3, 4, 5];

    // Borrowing with `&v` lets us keep using `v` after the loop.
    // Note: `for i in v` IS valid even without `mut`, but it MOVES `v` into the
    // loop, so `v` can no longer be used afterward.
    for i in &v {
        println!("{i}");
    }

    for i in v.iter() {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // Dereference mutable reference to modify value in-place
    }

    for idx in 0..v.len() {
        v[idx] += 50;
        print!("{} ", v[idx]);
    }
    println!();
}

fn enum_store() {
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn dropping_vector() {
    let v = vec![1, 2, 3, 4, 5];
    drop(v); // Explicitly frees the vector and all its elements
    // println!("{}", v[0]); // ERROR: use of moved value `v`

    {
        let _v = vec![1, 2, 3, 4, 5];
    } // `_v` goes out of scope and is automatically dropped here
}

fn main() {
    new_vector();
    reading_elements();
    borrow_checker_trap();
    enum_vec();
    iterating_over_vector();
    enum_store();
    dropping_vector();
}