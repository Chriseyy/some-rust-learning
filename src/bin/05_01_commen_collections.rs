

// Vector 

// Cretae a new vector

fn new_vector() {
    let v: Vec<i32> = Vec::new();  // vec holds i32 valus ... vector is also appendable, so we can add values to it later

    let v = vec![1, 2, 3]; // create a vector with initial values

    // update 
    //v.push(4);   // This will not work because v is immutable, we need to make it mutable/

    let mut v = Vec::new(); // create a mutable vector is i
    v.push(4); // Now we can add values to it
    v.push(5);

    // cant push string
    // v.push("Hello"); // This will not work because v is a vector of i32, we can only push i32 values to it  
    // need to declare a string vector or the first value in the vector should be a string, so that the vector can infer the type of the vector to be string

    // what i can pushi n vector without defining the type of the vector is the first value in the vector, so if the first value is a string, then we can push string values to it, but if the first value is an i32, then we can only push i32 values to it
}

fn enum_vec() {

    // this can for exmaple hold both i32 and String values, but we need to define an enum to hold both types of values
    enum MyType {
        Zahl(i32),
        Text(String),
    }

    let mut my_values: Vec<MyType> = Vec::new();
    my_values.push(MyType::Zahl(42));
    my_values.push(MyType::Text("Hello".to_string()));

    for value in my_values {
        match value {
            MyType::Zahl(n) => println!("Zahl: {}", n),
            MyType::Text(s) => println!("Text: {}", s),
        }
    }


    // or we define a vector wiht any type, but this is not recommended because we lose type safety and we can push any type of value to the vector, which can lead to runtime errors
    use std::any::Any;
    
    let mut alles_erlaubt: Vec<Box<dyn Any>> = Vec::new();

    alles_erlaubt.push(Box::new(42));
    alles_erlaubt.push(Box::new("Text"));
}

fn reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // get the third element of the vector
    println!("The third element is {}", third);

    match v.get(2) { // get the third element of the vector using get method
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // this doesnt work:
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // easy acces
    v.iter().for_each(|x| println!("{}", x)); // iterate over the vector and print each element
    println!("Vec pos 3 is {}", v[3]); // access the fourth element of the vector
}

fn interating_over_vector() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v { // iterate over the vector using a for loop
        println!("{}", i);
    }
    // we need to take &v because v is a vector of i32, and we want to iterate over the references to the elements of the vector, not the elements themselves. If we don't take a reference to v, we would be moving the elements out of the vector, which is not allowed because v is not mutable.

    for i in v.iter() { // iterate over the vector using the iter method
        println!("{}", i);
    }


    let mut v = vec![100, 32, 57];   // create a mutable vector
    for i in &mut v {
        *i += 50; 
    }
}


fn enum_store() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


fn dropping_vector() {
    let v = vec![1, 2, 3, 4, 5]; // create a vector
    drop(v); // drop the vector and free the memory
    // println!("{}", v[0]); // this will not work because v has been dropped and we cannot access it anymore

    // or out of scope 
    {
        let v = vec![1, 2, 3, 4, 5]; // create a vector
        // v goes out of scope and is dropped here
    }
}



fn main() {
    new_vector();
    reading_elements();
    enum_vec();
    interating_over_vector();
    enum_store();
    dropping_vector();
}