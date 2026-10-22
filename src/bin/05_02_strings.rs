
fn create_string() {
    let s = String::from("hello"); // create a string
    let s = "hello".to_string(); // create

    let mut s = String::new(); // create an empty string
    let data = "initial contents"; // create a string from a string literal
    let s = data.to_string(); // create a string from a string literal // the to_string() method is provided by the ToString trait, which is implemented for the str type, so we can call the to_string() method on a string literal to create a String
}

fn update_string() {
    let mut s = String::from("foo"); // create a mutable string
    s.push_str("bar"); // append a string slice to the string
    println!("{}", s); // print the string

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1.push_str(&s2); // append a string slice to the string
    println!("{}", s1); // print the string

    let mut s1 = String::from("foo"); // create a mutable string
    let s2 = "bar";  // create a string slice
    s1.push_str(s2);  // append a string slice to the string
    println!("s2 is {s2}");  // print the string slice, s2 is still valid because we only borrowed it, we did not move it, so we can still use it after the push_str() method call


    let mut s = String::from("lo");
    s.push('l'); // append a character to the string 
    println!("{}", s); // print the string
    // push vs push_str: push takes a single character as an argument, while push_str takes a string slice as an argument


    let s1 = String::from("Hello, ");  // not mutable, because we are going to move it to s3, so we cannot use it after the + operator call if we make it mutable, we can still use it after the + operator call, but we cannot use it after the + operator call if we make it mutable, because we are moving it to s3, so we cannot use it after the + operator call
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // the + operator uses the add method, which takes ownership of the first string and appends a copy of the second string to it, so s1 is no longer valid after this operation, but s2 is still valid because we only borrowed it, we did not move it, so we can still use it after the + operator call
    println!("{}", s3); // print the string

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    s1 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s1); // print the string

}


fn update_string_multi() {
    let mut s = String::from("tic"); // create a mutable string
    s.push_str("tac"); // append a string slice to the string
    s.push_str("toe"); // append a string slice to the string
    println!("{}", s); // print the string


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;  // we cant use s1 after this line because it has been moved, but we can still use s2 and s3 because we only borrowed them, we did not move them, so we can still use them after the + operator call
    println!("{}", s); // print the string

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // format! macro is more readable and does not take ownership of the strings, so we can still use them after the format! macro call
    println!("{}", s); // print the string
}

fn indexing_strings() {
    // Rust strings are UTF-8 encoded byte sequences. Therefore, you cannot use
    // indexing like s[i] to access a character: a Unicode scalar value can span
    // multiple bytes. Use chars() to get an iterator over the Unicode scalar values.

    let s1 = String::from("hello");
    // println!("{}", s1[0]); // Error: Rust strings cannot be indexed
    println!("{}", s1.chars().next().unwrap()); // Prints 'h'
    // println!("{}", s1[0..1]); // this will not work because strings are not indexed by bytes, but by characters, so we cannot access the first character of the string using the index operator, we need to use the chars() method to get an iterator over the characters of the string and then use the nth() method to get the first character of the string
    let h = &s1[0..1]; // this will work because we are taking a slice of the string, but
    println!("{}", h);  // containos first byte of the string, not the first character, so this will not work for strings that contain multi-byte characters, we need to use the chars() method to get an iterator over the characters of the string and then use the nth() method to get the first character of the string

    let hello = "Здравствуйте";  // this is a string literal, which is a string slice, so we can use the index operator to access the first character of the string, but we need to be careful because the first character of the string is not the first byte of the string, so we need to use the chars() method to get an iterator over the characters of the string and then use the nth() method to get the first character of the string
    let s = &hello[0..4];
    println!("{}", s);


    // iterate over string:
    for c in "Зд ".chars() {
        println!("{c}");
    }

    // iterate over string:
    for c in "Hell owrod ".chars() {
        println!("{c}");
    }


    // iterate of bytes of string:
    for b in "Зд".bytes() {
        println!("{b}");
    }


    // best way to access string index
    let s = String::from("hallo my friend");
    let s_chars: Vec<char> = s.chars().collect(); // collect the characters
    println!("{}", s_chars[0]); // access the first character of the string
    println!("{}", s_chars[5]); // access the sixth character of the string


    let s = String::from("hallo my friend");

    if let Some(ch) = s.chars().nth(5) {
        println!("{ch}"); // sixth Unicode scalar value
}
}

fn main()  {
    create_string();
    update_string();
    update_string_multi();
    indexing_strings();
}
