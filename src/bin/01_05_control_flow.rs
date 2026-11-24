// =============================================================================
// SECTION 1: IF EXPRESSIONS & BRANCHING
// In Rust, `if` is an expression (it evaluates to a value).
// Key rules:
// 1. The condition MUST evaluate strictly to a `bool`. Rust does NOT automatically 
//    convert integers or pointers to booleans (no "truthy" values like in JS/Python).
// 2. Multi-branch `if / else if / else` chains are evaluated sequentially.
// =============================================================================

fn if_example_1() {
    println!("=== 1. BASIC IF / ELSE ===");
    let number = 5;

    // `if number` would fail to compile because Rust requires an explicit boolean expression.
    if number < 10 {
        // True branch
        println!("The number is less than 10");
    } else {
        // False branch
        println!("The number is greater than or equal to 10");
    }
    println!("\n");
}

fn if_example_2() {
    println!("=== 2. ELSE IF CHAIN ===");
    let number = 5;

    if number < 10 {
        println!("The number is less than 10");
    } else if number == 10 {
        println!("The number is equal to 10");
    } else {
        println!("The number is greater than 10");
    }
    println!("\n");
}

fn if_example_3() {
    println!("=== 3. EQUALITY CHECKS ===");
    let number = 5;

    if number != 5 {
        println!("The number is not equal to 5");
    } else {
        println!("The number is equal to 5");
    }
    println!("\n");
}

// Handling multiple conditions with complex `else if` chains can get messy.
// In such cases, `match` expressions offer a cleaner alternative.
fn if_example_4() {
    println!("=== 4. MULTIPLE MODULO CHECKS ===");
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
    println!("\n");
}

// Because `if` is an expression, it can be assigned directly into a `let` statement.
fn if_example_5() {
    println!("=== 5. IF AS AN EXPRESSION IN ASSIGNMENTS ===");
    let condition = true;
    
    // Both arms must return the EXACT same type so the compiler can determine type at compile time.
    let number = if condition { 5 } else { 6 };

    // Invalid example:
    // let number = if condition { 5 } else { "six" };
    // ERROR: `if` and `else` have incompatible types (`i32` vs `&str`).

    println!("The value of number is: {number}");

    let number = 6;
    let is_divisible_by_4 = if number % 4 == 0 {
        true
    } else {
        false
    };

    println!("Is the number divisible by 4? {}", is_divisible_by_4);
    println!("\n");
}


// =============================================================================
// SECTION 2: REPETITION WITH `loop`
// `loop` creates an infinite loop until explicitly exited using `break` or `return`.
// Features:
// - `break`: Exits the loop immediately.
// - `continue`: Skips the remaining code in the current iteration.
// - Returning values: `break expression;` returns a value from the `loop`.
// =============================================================================

fn loop_example_1() {
    println!("=== 6. BASIC INFINITE LOOP & BREAK ===");
    let mut count = 0;

    loop {
        println!("Count: {count}");
        count += 1;

        if count >= 5 {
            break; // Exits loop when count reaches 5
        }
    }
    println!("\n");
}

fn loop_example_2() {
    println!("=== 7. LOOP WITH CONTINUE ===");
    let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            continue; // Skips printing when count is 3
        }
        println!("Count: {count}");
        if count >= 5 {
            break;
        }
    }
    println!("\n");
}

// Returning values directly from a loop via `break value;`
fn loop_example_3() {
    println!("=== 8. RETURNING VALUES FROM A LOOP ===");
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // Returns 20 to `result` upon break
        }
    };

    println!("The result is: {result}");
    println!("\n");
}

// Difference between `break` and `return`:
// `break` exits only the loop; `return` exits the ENTIRE function immediately.
fn loop_example_4() -> i32 {
    let mut count = 0;

    let _result = loop {
        count += 1;

        if count == 10 {
            return count * 2; // Exits the function early, returning 20
        }
    };

    // Unreachable code (compiler will issue a warning if un-commented)
    // println!("This will never print!");
}

// Disambiguation with Loop Labels ('label:)
// Labels allow `break` or `continue` to target outer loops specifically.
fn loop_example_5() {
    println!("=== 9. LABELED LOOPS ===");
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Breaks inner loop
            }
            if count == 2 {
                break 'counting_up; // Breaks outer loop labeled `'counting_up`
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    println!("\n");
}


// =============================================================================
// SECTION 3: CONDITIONAL LOOPS WITH `while`
// `while` executes as long as a boolean condition evaluates to `true`.
// Eliminates boilerplate `loop` + `if` + `break` patterns.
// =============================================================================

fn while_example_1() {
    println!("=== 10. WHILE LOOP BASICS ===");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!("\n");
}

fn while_example_2() {
    println!("=== 11. WHILE LOOP WITH BREAK ===");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        if number == 2 {
            break; // Early exit from while loop
        }

        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!("\n");
}


// =============================================================================
// SECTION 4: ITERATING WITH `for` LOOPS
// `for` loops are the safest and most idiomatic way to iterate over collections
// (arrays, vectors) or numerical ranges in Rust.
// Prevents out-of-bounds indexing bugs inherent to manual index counters.
// =============================================================================

fn for_example_1() {
    println!("=== 12. FOR LOOP OVER ARRAY ===");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    println!("\n");
}

fn for_example_2() {
    println!("=== 13. FOR LOOP USING .iter() ===");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {element}");
    }
    println!("\n");
}

fn for_example_3() {
    println!("=== 14. FOR LOOP WITH RANGES ===");
    // `1..=5` is an inclusive range (1, 2, 3, 4, 5).
    // `1..5` would be an exclusive range (1, 2, 3, 4).
    for i in 1..=5 {
        println!("the value is: {i}");
    }
    println!("\n");
}

fn for_example_4() {
    println!("=== 15. FOR LOOP WITH ENUMERATE ===");
    let a = [10, 20, 30, 40, 50];

    // `.enumerate()` yields pairs of `(index, &value)`
    for (index, value) in a.iter().enumerate() {
        println!("the value at index {index} is: {value}");
    }
    println!("\n");
}

fn for_example_5_reverse() {
    println!("=== 16. REVERSED ITERATION (.rev) ===");
    let a = [10, 20, 30, 40, 50];

    // `.rev()` reverses the iteration order
    for element in a.iter().rev() {
        println!("the value is: {element}");
    }
    println!("\n");
}

fn for_exmple_6_while() {
    println!("=== 17. MANUAL ARRAY ITERATION WITH WHILE ===");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Less idiomatic: prone to runtime panics if boundary logic is flawed
    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    println!("\n");
}


// =============================================================================
// SECTION 5: PATTERN MATCHING WITH `match`
// `match` checks a value against sequential patterns.
// Supports multiple values (`|`), ranges (`..=`), and wildcards (`_`).
// =============================================================================

fn match_example() {
    println!("=== 18. MATCH CONTROL FLOW ===");
    let number = 3;

    match number {
        1 => println!("Eins!"),
        2 | 3 => println!("Zwei oder Drei!"),        // Multiple pattern matching
        4..=10 => println!("Zwischen Vier und Zehn!"), // Inclusive range pattern
        _ => println!("Irgendeine andere Zahl!"),      // Catch-all placeholder
    }
    println!("\n");
}


// =============================================================================
// SECTION 6: CONCISE PATTERN MATCHING WITH `if let` & `while let`
// Concise alternatives when you only care about matching ONE specific variant
// (e.g., `Some(T)` in an `Option`) and ignoring remaining possibilities.
// =============================================================================

fn if_let_example() {
    println!("=== 19. IF LET DEMO ===");
    let some_option = Some(5);

    if let Some(value) = some_option {
        println!("The value is: {value}");
    } else {
        println!("The value is None");
    }
    println!("\n");
}

fn while_let_example() {
    println!("=== 20. WHILE LET DEMO ===");
    let mut some_option = Some(0);

    // Continuously runs as long as `some_option` matches `Some(value)`
    while let Some(value) = some_option {
        println!("The value is: {value}");
        if value >= 5 {
            some_option = None; // Exits loop by resetting `some_option` to `None`
        } else {
            some_option = Some(value + 1);
        }
    }

    println!("The value is None");
    println!("\n");
}


// =============================================================================
// SECTION 7: TESTING & ASSERTION MACROS DEMO
// Common macros used for testing and validating control flow conditions.
// =============================================================================

fn testing_macros_demo() {
    println!("=== 21. TESTING MACROS DEMO ===");
    
    // `assert!`: Panics if the expression evaluates to `false`
    assert!(true);

    // `assert_eq!`: Panics if left != right
    assert_eq!(2 + 2, 4);

    // `assert_ne!`: Panics if left == right
    assert_ne!(5, 10);

    println!("All assertions passed successfully!");
    println!("\n");
}


// =============================================================================
// MAIN ENTRY POINT
// Guaranteed to run cleanly without errors.
// =============================================================================

fn main() {
    if_example_1();
    if_example_2();
    if_example_3();
    if_example_4();
    if_example_5();

    loop_example_1();
    loop_example_2();
    loop_example_3();
    
    let count = loop_example_4();
    println!("The result from loop_example_4 is: {count}\n");
    
    loop_example_5();

    while_example_1();
    while_example_2();

    for_example_1();
    for_example_2();
    for_example_3();
    for_example_4();
    for_example_5_reverse();
    for_exmple_6_while();

    match_example();

    if_let_example();
    while_let_example();
    
    testing_macros_demo();
}

/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. IF EXPRESSIONS vs. MATCH EXPRESSIONS
---------------------------------------
- Use `if` for simple boolean checks or two-way conditional assignments.
- Use `match` for complex multi-branch conditions, range checks, enum variants,
  or when exhaustive compiler checking is required.


2. CHOOSING THE RIGHT LOOP CONSTRUCT
------------------------------------
- `loop`: Best for infinite loops (e.g., servers, game loops) or when you need 
  to return a value from a loop using `break value;`.
- `while`: Best for repeating execution based on a single changing boolean condition.
- `for`: Preferred idiomatic choice for iterating over arrays, collections, ranges, 
  or iterators. It eliminates out-of-bounds indexing panics at runtime.


3. LOOP LABELS & SCOPE CONTROLS
-------------------------------
- Use loop labels (`'label_name: loop`) when nesting loops to explicitly target 
  which outer loop to `break` or `continue`.
- `break` exits the current loop; `return` exits the surrounding function entirely.


4. CONCISE MATCHING WITH `if let` & `while let`
-----------------------------------------------
- Use `if let` or `while let` when you want to handle a single enum pattern 
  (like `Some(val)`) without writing a full, verbose `match` block with `_ => ()`.


5. CONTROL FLOW MACROS FOR SAFETY
---------------------------------
- `panic!("msg")`: Immediately crashes the thread with an error message.
- `assert!(cond)`: Panics if condition is false.
- `assert_eq!(a, b)`: Panics if `a != b` (useful in tests).
- `assert_ne!(a, b)`: Panics if `a == b`.
================================================================================
*/