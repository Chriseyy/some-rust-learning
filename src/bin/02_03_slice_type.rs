

// Slice type
// this means how to use slice type in rust
// slice type is a dynamically sized view into a contiguous sequence of elements in a collection, such as an array or a vector. 
// It allows you to work with a portion of the collection without needing to copy the data. 
//In Rust, slices are represented by the `&[T]` type, where `T` is the type of the elements in the slice.


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// What happen here:
// The function `first_word` takes a reference to a `String` as an argument and returns the index of the first space character in the string.
// We iterate over the bytes of the string using `as_bytes()` and `iter().enumerate()`, which gives us both the index and the byte value of each character.
// example: byte value of ' ' is 32, so we compare each byte to `b' '` (the byte representation of a space character).

// how do we get the second word?
fn second_word(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes();

    let mut first_space_index = None;
    let mut second_space_index = None;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_space_index.is_none() {
                first_space_index = Some(i);  // Some i means that we have found the first space, so we store its index in `first_space_index`.
                                                // some() is an enum variant that represents a value that may or may not be present. In this case, we use it to indicate whether we have found the first space or not.
            } else {
                second_space_index = Some(i);
                break;
            }
        }
    }

    match (first_space_index, second_space_index) {
        (Some(first), Some(second)) => (first, second),
        (Some(first), None) => (first, s.len()),
        _ => (0, 0), // No spaces found
    }
}


// String slices 

// fn string_slice_example() -> (&str, &str) { // this casue an error because we are returning a reference to a local variable, which will be dropped at the end of the function.
// fn string_slice_example(s: &str) -> (&str, &str) { // this is the correct way to do it, we are passing a reference to the string as an argument, so we can return a reference to a slice of that string.
fn string_slice_example() -> (String, String) { // other way to do it is to return a tuple of two strings, which will be owned by the caller, so we can return a reference to a slice of that string.
    let my_string = String::from("Hello, world!");
    let slice = &my_string[0..5]; // This creates a slice of the first 5 characters of the string
    let slice2 = &my_string[7..12]; // This creates a slice of the characters from index 7 to 12 (exclusive)
    return (slice.to_string(), slice2.to_string()) // whit the to_string() method we are converting the slice to a string, so we can return it as a string. we create objects of type String, so we can return them as owned values, which will be owned by the caller, so we can return a reference to a slice of that string.
    // (slice, slice2)  // no to stirng is this fn string_slice_example(s: &str) -> (&str, &str) the function
}

fn get_length(s: &str) -> usize {
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice_array(arr: &[i32]) -> &[i32] {
    &arr[1..3] // This creates a slice of the array from index 1 to 3 (exclusive)
}

fn main() {
    let mut my_string = String::from("Hello, world!");
    let index = first_word(&my_string);
    println!("The index of the first space is: {}", index);
    println!("The first word is: {}", &my_string[..index]); // This will print "Hello,"
    my_string.clear(); // This will clear the my_string variable
    // cause of clear we need to initialize the my_string again, so we can get the first word again.
    // this wouldnt work &my_string[..index] cause the string is gone but index is still there, so we need to initialize the my_string again, so we can get the first word again.
    // string is gone but index is still there, so we need to initialize the my_string again, so we can get the first word again.
    
    let my_string = String::from("Hello, world!");
    let (first_space, second_space) = second_word(&my_string);
    println!("The index of the first space is: {}", first_space);
    println!("The index of the second space is: {}", second_space);

     
    let (slice, slice2) = string_slice_example();
    println!("The first slice is: {}", slice);
    println!("The second slice is: {}", slice2);

    let length = get_length(&my_string);
    println!("The length of the string is: {}", length);

    let first_word = first_word_slice(&my_string);
    println!("The first word is: {}", first_word);

    let my_array = [1, 2, 3, 4, 5];
    let slice = slice_array(&my_array);
    println!("The slice of the array is: {:?}", slice);
}
