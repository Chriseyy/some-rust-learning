
// Functions

fn another_function() {
    println!("Another function.");
} 
// define a functino with the fn keyword, followed by the name of the function, and a pair of parentheses that may optionally contain parameters.
// The body of the function is contained within a pair of curly braces.

// you can define it before or after the main function, as long as it is in the same scope. 
// scope is the area of the code where a variable or function is valid and can be accessed.



// Functions with Parameters
// Function can have multiple parameters, and they are separated by commas in the function definition.
fn another_function_with_parameters(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn some_random_text_with_parameters(text: &str) {
    println!("This is some random text: {}", text);
}
// Functions can also have parameters, which are values that are passed into the function when it is called.
// Parameters are specified in the parentheses after the function name, and each parameter has a name and a type annotation. 
// The type annotation specifies the type of the parameter, and it is required for all parameters in a function. 
// When you call a function with parameters, you need to provide arguments that match the types of the parameters in the function definition. 
// The arguments are the values that are passed into the function when it is called, and they are used to perform the operations defined in the function body.

// here text is "Hello, world!" and it is passed as an argument to the some_random_text_with_parameters function when it is called in the main function.

// fn paramter needs to have a type annotation, and the type of the argument passed to the function must match the type of the parameter in the function definition.

// fn stuff2(x) {
//     println!("The value of x is: {x}");
// }
// wont compile because the parameter x does not have a type annotation, and the compiler cannot infer the type of x based on the value passed to the function when it is called.

// parameters are also immutable by default, which means that you cannot change the value of a parameter within the function body.
// If you want to change the value of a parameter, you need to declare it as mutable using the mut keyword in the function definition, and then you can change its value within the function body

// Parameters can be arrays, tuples, or any other type of data that can be passed as an argument to a function.




// Statements and Expressions
// A statement is an instruction that performs some action and does not return a value.
// An expression is a piece of code that evaluates to a value.
// For example, let x = 5; is a statement because it performs the action of declaring a variable and assigning it a value, but it does not return a value. 
// On the other hand, 5 + 6 is an expression because it evaluates to a value (11) and can be used in a statement like let x = 5 + 6; where the expression 5 + 6 is evaluated and its value is assigned to the variable x.
// In Rust, the main function is a special function that serves as the entry point of the program. When you run a Rust program, the main function is the first function that is executed, and it is where you can call other functions and perform the operations defined in your program. 
// The main function is defined using the fn keyword, followed by the name main, and it does not take any parameters or return any values. 
// The body of the main function is contained within a pair of curly braces, and it is where you can write the code that will be executed when the program runs.

fn add_stuff() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
// In this example, the block of code inside the curly braces is an expression that evaluates to a value. 
// The value of the expression is the value of the last expression in the block, which is x + 1. 
// The value of x is 3, so the value of the expression is 4, and that value is assigned to the variable y. 
// The block of code is an expression because it evaluates to a value, and it can be used in a statement like let y = { ... }; where the value of the block is assigned to the variable y.






// Return Values
// Functions can also return values, which are the results of the operations performed in the function body. 
// To return a value from a function, you can use the return keyword followed by the value you want to return. 
// The return type of a function is specified in the function definition after the parameter list, using the -> syntax. 
// For example, fn add(x: i32, y: i32) -> i32 { ... } defines a function that takes two parameters of type i32 and returns a value of type i32. 
// The return type is optional, and if it is not specified, the function will return the unit type (), which is an empty tuple that represents the absence of a value. 
// When a function returns a value, you can use that value in the code that calls the function, and you can also assign it to a variable. 
// For example, let result = add(5, 10); calls the add function with the arguments 5 and 10, and assigns the return value of the function to the variable result. 
// You can then use the value of result in your code, such as printing it to the console or performing further operations with it.  

fn number_plus_one(x: i32) -> i32 {
    x + 1
}
// In this example, the number_plus_one function takes a parameter x of type i32 and returns a value of type i32. 
// The function body consists of a single expression, x + 1, which evaluates to the value of x plus 1. The return type of the function

// in main function, we can call the number_plus_one function and use its return value like this:
// let result = number_plus_one(5);
// println!("The result is: {result}");
// This will call the number_plus_one function with the argument 5, and the return value will be 6, which is then assigned to the variable result and printed to the console.   
// if no let in main function, we can also call the number_plus_one function and use its return value like this:
// println!("The result is: {}", number_plus_one(5));
// This will call the number_plus_one function with the argument 5, and the return value will be 6, which is then printed to the console directly without being assigned to a variable.

// with ; at the end of the expression, the function will return the unit type () instead of the value of the expression.
// fn number_plus_one(x: i32) -> i32 {  
//     x + 1; // this will return the unit type () instead of the value of x + 1
// }
// In this example, the number_plus_one function will return the unit type () instead of the value of x + 1 because the expression x + 1 is followed by a semicolon,
// which turns it into a statement that does not return a value.
// to return the value of x + 1, we need to remove the semicolon at the end of the expression like before 

// return x+1 is also valid, but it is not idiomatic Rust code.
// In Rust, the last expression in a function is automatically returned, so you can simply write x + 1 without the return keyword, and it will be returned as the value of the function. 
// Using the return keyword is not necessary in this case, and it can make the code less concise and less readable. 
// Import for early returns, where you want to return a value from the middle of a function before reaching the end of the function body.
fn check_age(age: i32) -> String {
    // early return if the age is less than 18
    if age < 18 {
        return String::from("TOOO YOUNG"); 
    }
    
    println!("Age passed");
    // return a string if the age is 18 or older normal return without return keyword, because the last expression in the function is automatically returned
    String::from("HELLLLOO!") 
}


fn return_string_example() -> String {
    String::from("Hello, world!")
}
// String example of a function that returns a String value. The return type of the function is specified as String, and the function body creates a new String value using the String::from function and returns it.
// When you call the return_string_example function, it will return the String value "Hello, world!"

// you can also return arrays or all other types liks structs, tuples, etc. from a function by specifying the appropriate return type in the function definition and returning the desired value from the function body.
// or multiple values using tuples, for example:
fn get_user_info() -> (String, i32) {
    let name = String::from("Alice");
    let age = 30;
    (name, age) 
}

struct User {
    name: String,
    age: i32,
}
fn create_user() -> User {
    User {
        name: String::from("Chris"),
        age: 30,
    }
}
// complex example

// some import stuff -> gives return type of the function, and the value of the last expression in the function body is returned as the return value of the function.
// if not specified, the return type of the function is () which is the unit type that represents the absence of a value.
// if type wrong error, the compiler will give an error because the return type of the function does not match the type of the value being returned in the function body.
// fn get_name() -> String {
//     42 // ERROR: expected `String`, found integer
// }
// if no -> for return error:
// fn get_number() {  // Compiler denkt: -> ()
//     5 // error: expected `()`, found integer
//     same with return 5; 
// }
// what works is print stuff:
// fn get_number() {  // Compiler thinks: -> ()
//     println!("The number is: {}", 5); // this will print the number to the console, but it will not return a value from the function, and the return type of the function will be () which is the unit type that represents the absence of a value.
// }
// but you can use return in function 

// Early Returns
fn say_hello(name: &str) { 
    if name == "" {
        println!("No name provided!");
        return; 
    }
    
    println!("Hello, {}!", name);
}
// In this example, the sag_hallo function takes a parameter name of type &str and does not return any value (return type is ()), but it uses an early return to exit the function if the name parameter is an empty string. 
// If the name parameter is an empty string, the function will print a message to the console and then return immediately, without executing the rest of the function body. 
// If the name parameter is not an empty string, the function will print a greeting message to the console that includes the name parameter.



fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(5, 10);
    some_random_text_with_parameters("Hello, world!");
    add_stuff();
    let result = number_plus_one(5);
    println!("The result is: {result}");
    number_plus_one(1); // this will call the number_plus_one function with the argument 1, but the return value will not be used, and the compiler will warn you about unused return values.
    return_string_example(); // this will call the return_string_example function, but the return value will not be used, and the compiler will warn you about unused return values.
    // doesn't show up in terminal because the return value of the function is not used, and the compiler will warn you about unused return values.

    let age = 17;
    let age_check = check_age(age);
    println!("Age check: {age_check}");


    let str2 = return_string_example(); // this will call the return_string_example function and assign its return value to the variable str, which can then be used in the code that follows.
    println!("The value of str is: {str2}");

    let user = create_user(); // this will call the create_user function and assign its return value, which is a User struct, to the variable user. You can then access the fields of the User struct using dot notation, like user.name and user.age.
    println!("User name: {}, User age: {}", user.name, user.age);

    let user_info = get_user_info();  // destructuring the tuple returned by the get_user_info function into separate variables name and age
    let (name, age) = get_user_info(); // this will call the get_user_info function and destructure the returned tuple into separate variables name and age.
    println!("User info: {:?}", user_info);
    println!("Name: {}, Age: {}", name, age);

    say_hello(""); // this will call the say_hello function with an empty string as the argument, which will trigger the early return and print "No name provided!" to the console.
    say_hello("Alice"); // this will call the say_hello function with the argument
}
