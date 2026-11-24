
// Error handeling in Rust

// unrecoverbale errore with panic !
// this means that the program will stop execution and print an error message to the console. This is useful for situations where the program cannot continue, such as when a file is not found or when a network connection is lost.


fn vec_example() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // this will panic if the index is out of bounds
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2); // this will return None if the index is out of bounds
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // this will panic if the index is out of bounds
    let does_not_exist = &v[100]; // this will panic if the index is out of bounds
    println!("The 100th element is {}", does_not_exist);
}


// other examples of panic in Rust are when we try to unwrap an Option that is None, or when we try to unwrap a Result that is Err. In these cases, the program will stop execution and print an error message to the console.
fn unwrap_example() {
    let some_option: Option<i32> = None;
    let some_value = some_option.unwrap(); // this will panic because the Option is None
    println!("The value is {}", some_value);
}

fn  main() {
    println!("Hello, world!");
    vec_example();  // auto error panic if index is out of bounds, but we can handle it with Option and match
    unwrap_example(); // this will panic because the Option is None
}
