
//Controlflow

// Controlflow is the way that a program decides what code to execute next. 
// It is the way that a program can make decisions, repeat code, and handle errors. In Rust, there are several control flow constructs, including if statements, loops, and match statements.

// if expressions are used to make decisions in Rust. 
// They allow you to execute different code based on a condition. The syntax for an if expression is as follows:

fn if_example_1() {
    let number = 5;

    // if number // would check for bool and not work, we need to compare it to something

    if number < 10 {
        // True branch
        println!("The number is less than 10");
    } else {
        // False branch
        println!("The number is greater than or equal to 10");
    }
}


// else if expressions are used to check multiple conditions in a sequence.

fn if_example_2() {
    let number = 5;

    if number < 10 {
        println!("The number is less than 10");
    } else if number == 10 {
        println!("The number is equal to 10");
    } else {
        println!("The number is greater than 10");
    }
}


fn if_example_3() {
    let number = 5;

    if number != 5 {
        println!("The number is not equal to 5");
    }
    else {
        println!("The number is equal to 5");
    }
}

// Handeling mutliple conditions with if statements can get messy, and it can be hard to read. In those cases, it is often better to use a match statement, which is a more powerful and flexible control flow construct. 
fn if_example_4() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// because if is and expression, it can be used in a let statement to assign a value to a variable based on a condition.

fn if_example_5() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // let number = if condition { 5 } else { "six" };
    // this would not work because the if and else branches must return the same type, in this case, they both return an integer.
    // Rust needs to know definitively at compile time what type the number variable is.

    println!("The value of number is: {number}");

    //--- 

    let number = 6;

    let is_divisible_by_4 = if number % 4 == 0 {
        true
    } else {
        false
    };

    println!("Is the number divisible by 4? {}", is_divisible_by_4);
}


// -------------------------

// Repetion with Loops

// loop while and for loops are used to repeat code in Rust.

// loop is an infinite loop that will run until it is explicitly told to stop. It is often used for programs that need to run indefinitely, such as servers or games.
// without a break statement, this loop would run forever, so we need to use a break statement to exit the loop when a certain condition is met.

// break to exit early
fn loop_example_1() {
    let mut count = 0;

    loop {
        println!("Count: {count}");
        count += 1;

        if count >= 5 {
            break; // this will exit the loop when count is 5 or greater
        }
    }
}

// continue example - this will skip the rest of the current iteration and move on to the next iteration of the loop.
fn loop_example_2() {
    let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            continue; // this will skip the rest of the current iteration and move on to the next iteration
        }
        println!("Count: {count}");
        if count >= 5 {
            break;
        }
    }
}


// Return values froms loops
// loops can also return values in Rust. This is done by using the break statement with a value. The value that is returned from the loop will be the value that is passed to the break statement.

fn loop_example_3() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // this will return the value of count * 2 when the loop is exited
        }
    };

    println!("The result is: {result}"); // prints "The result is: 20" because when count is 10, count * 2 is 20, and that value is returned from the loop and assigned to the result variable.
}

// but you can also return values with return statements, but this is less common and not recommended because it can make the code harder to read and understand.

fn loop_example_4() -> i32 {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            return count * 2; // this will return the value of count * 2 and exit the entire function, not just the loop
        }
    };

    // this code will never be reached because the return statement will exit the function before it is executed.
    // println!("The result is: {result}");
}

// difference between break and return in loops:
// You can also return from inside a loop. While break only exits the current loop, return always exits the current function.
// break will only exit the loop, and the code after the loop will still be executed. return will exit the entire function, and the code after the loop will not be executed.


// Disambiguation wiht loop labels

fn loop_example_5() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
// In this example, we have an outer loop labeled 'counting_up and an inner loop. The inner loop will break when remaining is 9, but the outer loop will only break when count is 2. The output of this code will be:
// count = 0
// remaining = 10
// remaining = 9


// --------------------------------------

// Streamlining Conditional Loops with while
// While loops are used to repeat code while a certain condition is true. The syntax for a while loop is as follows:


fn while_example_1() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
// This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer
//  While a condition evaluates to true, the code runs; otherwise, it exits the loop.
// you can also leave with break and return from a while loop, just like with a loop, but it is less common and not recommended because it can make the code harder to read and understand.

fn while_example_2() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        if number == 2 {
            break; // this will exit the loop when number is 2
        }

        number -= 1;
    }

    println!("LIFTOFF!!!");
}



// For loop 
// iterating over a collection of items, such as an array or a vector. The syntax for a for loop is as follows:

fn for_example_1() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_example_2() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {element}");
    }
}
// .iter means that we are iterating over the elements of the array, rather than the indices. This is often more convenient and less error-prone than using a while loop with an index variable.


fn for_example_3() {
    // for i in range(1, 6) { // this would be the equivalent of 1..=5 in Rust, because the end value is exclusive in Python and inclusive in Rust.
    for i in 1..=5 {
        println!("the value is: {i}");
    }
}
// 1..=5 is a range that includes the numbers from 1 to 5, inclusive. This is a convenient way to iterate over a range of numbers without having to create an array or vector.


fn for_example_4() {
    let a = [10, 20, 30, 40, 50];

    // python for index, value in enumerate(a):
    for (index, value) in a.iter().enumerate() {
        println!("the value at index {index} is: {value}");
    }
}
//.iter().enumerate() is a method that returns an iterator that yields pairs of the index and the value of each element in the array. This is useful when you need to access both the index and the value of each element in the array.
// in pyhton also for index, value in enumerate(a): 


fn for_example_5_reverse() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter().rev() {
        println!("the value is: {element}");
    }
}
// .rev() is a method that returns an iterator that yields the elements of the array in reverse order. This is useful when you need to iterate over the elements of the array in reverse order.
// basically interation reverse through array, so it would print 50, 40, 30, 20, 10 instead of 10, 20, 30, 40, 50.

fn for_exmple_6_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
// we need mut index because we need to change the value of index in each iteration of the loop. This is a more manual way to iterate over the elements of the array, and it is less convenient and more error-prone than using a for loop with .iter() or a range.


// match statements
// match statements are another powerful control flow construct in Rust, but they are not covered in this example. They allow you to match a value against a series of patterns and execute code based on which pattern matches. They are often used for handling complex data structures and for error handling.

fn match_example() {
    let number = 3;

    match number {
        1 => println!("Eins!"),
        2 | 3 => println!("Zwei oder Drei!"), // multiple patterns can be combined with the | operator
        4..=10 => println!("Zwischen Vier und Zehn!"), // ranges can be used to match a range of values
        _ => println!("Irgendeine andere Zahl!"), // "_" is a catch-all pattern that matches any value that hasn't been matched by the previous patterns
    }
}
// enums like Option and Result are often used with match statements to handle cases where a value may be present or absent, or where an operation may succeed or fail. This allows for more robust error handling and more expressive code.
// it is a if alternative that is more powerful and flexible, but it can also be more complex and harder to read if used improperly. It is important to use match statements judiciously and to keep the patterns simple and easy to understand.



// if_let and while_let are also control flow constructs that are used to handle cases where a value may be present or absent, but they are not covered in this example. They allow you to match a value against a pattern and execute code if the pattern matches, while ignoring the case where the pattern does not match. They are often used with enums like Option and Result to handle cases where a value may be present or absent, or where an operation may succeed or fail.

// Some() is an enum variant of the Option type that represents a value that is present. It is often used in conjunction with if_let and while_let to handle cases where a value may be present or absent.
// Example Some(5) is an instance of the Option type that represents a value of 5 that is present. It can be used in an if_let expression to check if the value is present and to extract the value if it is present.
fn if_let_example() {
    let some_option = Some(5);

    if let Some(value) = some_option {
        println!("The value is: {value}");
    } else {
        println!("The value is None");
    }
}

fn while_let_example() {
    let mut some_option = Some(0);

    while let Some(value) = some_option {
        println!("The value is: {value}");
        if value >= 5 {
            some_option = None; // this will exit the loop when the value is 5 or greater
        } else {
            some_option = Some(value + 1); // this will increment the value and continue the loop
        }
    }

    println!("The value is None");
}

fn main() {
    if_example_1();
    if_example_2();
    if_example_3();
    if_example_4();
    if_example_5();

    println!("---");
    loop_example_1();
    loop_example_2();
    loop_example_3();
    let count = loop_example_4();
    println!("The result from loop_example_4 is: {count}");
    loop_example_5();

    println!("---");
    while_example_1();
    while_example_2();

    println!("---");
    for_example_1();
    println!("-");
    for_example_2();
    println!("-");
    for_example_3();
    println!("-");
    for_example_4();
    println!("-");
    for_example_5_reverse();
    println!("-");
    for_exmple_6_while();

    println!("---");
    match_example();

    println!("---");
    if_let_example();
    while_let_example();
}





// Other functions that can be used with loops and control flow constructs include:
// - break: exits the current loop
// - continue: skips the rest of the current iteration and moves on to the next iteration
// - return: exits the current function and returns a value
// - panic!: causes the program to crash with an error message
// - assert!: checks a condition and panics if the condition is false
// - assert_eq!: checks that two values are equal and panics if they are not
// - assert_ne!: checks that two values are not equal and panics if they are equal

