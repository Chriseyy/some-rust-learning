// Error handling in Rust: Common causes of `panic!`
//
// Rust panics automatically on unrecoverable bugs like out-of-bounds indexing
// or calling `.unwrap()` on `None` / `Err`.
// (For expected failures like missing files or bad user input, use `Result`.)
//
// Debugging tip: Run with a backtrace to see the exact call stack on panic:
//   RUST_BACKTRACE=1 cargo run

fn vec_example() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // Panics if index is out of bounds
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // Safely returns None if out of bounds
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Panics at runtime: index out of bounds (len is 5, index is 100)
    let does_not_exist = &v[100];
    println!("The 100th element is {does_not_exist}");
}

// Calling `.unwrap()` on `None` (or `Err`) also triggers a panic.
fn unwrap_example() {
    let some_option: Option<i32> = None;
    let some_value = some_option.unwrap(); // Panics here
    println!("The value is {some_value}");
}

fn main() {
    println!("Hello, world!");
    vec_example();    // Panics on &v[100]
    unwrap_example(); // Never reached unless vec_example() is commented out
}