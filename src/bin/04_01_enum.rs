// =============================================================================
// SECTION 1: ENUMS VS. STRUCTS (DATA ATTACHMENT)
// Enums represent an "OR" relationship (a value is Option A OR Option B).
// Structs represent an "AND" relationship (a value has Field A AND Field B).
// In Rust, you can attach data directly into enum variants, avoiding extra structs.
// =============================================================================

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Approach A: Separate struct holding an enum tag + String address data.
// This is verbose because you need two separate types to represent one concept.
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

// Approach B: Direct payload inside the enum variants (Recommended/Idiomatic).
// Each variant can hold different types and amounts of data directly.
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // Holds four 8-bit integers
    V6(String),          // Holds a single String
}

fn demo_ip_addresses() {
    println!("=== 1. IP ADDRESSES DEMO ===");

    // Instantiating basic enum variants without data
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Simple enum kinds: {:?}, {:?}", four, six);

    // Creating an instance using the Struct + Enum pattern
    let home_struct = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("Struct IP Kind: {:?}", home_struct.kind);

    // Creating instances with attached data directly via constructors
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Direct Enum IPs: {:?}, {:?}", home, loopback);
    println!("\n");
}


// =============================================================================
// SECTION 2: COMPLEX ENUM VARIANTS & METHODS
// Rust enums can mix unit variants, struct-like variants, and tuple variants.
// Like structs, enums can also have methods defined on them using `impl`.
// =============================================================================

#[derive(Debug)]
enum Message {
    Quit,                       // No attached data (Unit variant)
    Move { x: i32, y: i32 },    // Named fields (Struct-like variant)
    Write(String),              // Single String payload (Tuple variant)
    ChangeColor(i32, i32, i32), // Three i32 values (Tuple variant)
}

impl Message {
    // A method on an enum uses `self` to inspect which variant called it.
    fn call(&self) {
        println!("Message called on: {:?}", self);
    }
}

fn demo_message_enum() {
    println!("=== 2. MESSAGE ENUM DEMO ===");
    let msg = Message::Write(String::from("Hello from Rust!"));
    msg.call();
    println!("\n");
}


// =============================================================================
// SECTION 3: MATCH CONTROL FLOW & EXHAUSTIVENESS
// `match` evaluates a value against patterns sequentially.
// Key Rule: `match` MUST cover every single possible variant (Exhaustive).
// You can extract (bind) internal data inside a match arm.
// =============================================================================

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Holds nested UsState data
}

fn value_in_cents(coin: &Coin) -> u8 {
    // The compiler checks that all variants of Coin are handled here.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // 'state' extracts/binds the inner UsState data from Quarter
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn demo_dice_roll(dice_roll: u8) {
    // Catch-all with variable binding ('other' captures any unhandled number)
    match dice_roll {
        3 => println!("You get a hat!"),
        7 => println!("You lose a hat!"),
        other => println!("Move {} spaces.", other),
    }

    // Catch-all with '_' placeholder (ignores the value without binding)
    match dice_roll {
        3 => println!("You get a hat!"),
        7 => println!("You lose a hat!"),
        _ => println!("Reroll the dice!"),
    }

    // Catch-all returning unit `()` (does absolutely nothing for other cases)
    match dice_roll {
        3 => println!("You get a hat!"),
        7 => println!("You lose a hat!"),
        _ => (),
    }
}

fn demo_match_flow() {
    println!("=== 3. MATCH CONTROL FLOW ===");
    let coin = Coin::Quarter(UsState::Alaska);
    let cents = value_in_cents(&coin);
    println!("Value in cents: {}", cents);

    demo_dice_roll(9);
    println!("\n");
}


// =============================================================================
// SECTION 4: THE OPTION<T> ENUM & NULL SAFETY
// Rust eliminates null pointers by using `Option<T>` (`Some(T)` or `None`).
// Because `Option<T>` is a different type than `T`, the compiler forces you
// to extract the value safely before performing operations on it.
// =============================================================================

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,            // Safely returns None if no value exists
        Some(i) => Some(i + 1),  // Extracts 'i', adds 1, and wraps it back in Some
    }
}

fn demo_option() {
    println!("=== 4. OPTION<T> ENUM ===");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);
    println!("\n");
}


// =============================================================================
// SECTION 5: CONCISE CONTROL FLOW (IF LET & LET...ELSE)
// `if let` provides a shortcut when you only care about ONE pattern and ignore rest.
// `let...else` extracts data and keeps execution on the "happy path" by requiring
// an early return/break inside the else block if matching fails.
// =============================================================================

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // `let...else` binds 'state' if matching succeeds.
    // If it is not a Quarter, execution enters the else block and returns early.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // 'state' is now available in the outer scope without deep indentation.
    if state.existed_in(1900) {
        Some(format!("{:?} is pretty old!", state))
    } else {
        Some(format!("{:?} is relatively new.", state))
    }
}

fn demo_concise_control_flow() {
    println!("=== 5. IF LET & LET...ELSE ===");

    // Basic `if let` syntax (replaces a match that only has one useful arm)
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // `if let` combined with an `else` branch
    let mut count = 0;
    let coin = Coin::Dime;

    if let Coin::Quarter(state) = &coin {
        println!("Quarter from {:?}!", state);
    } else {
        count += 1;
        println!("Not a quarter! Count increased to: {}", count);
    }

    // Testing `let...else` early-exit flow
    let q_alaska = Coin::Quarter(UsState::Alaska);
    let result = describe_state_quarter(q_alaska);
    println!("State Quarter Status: {:?}", result);

    let dime = Coin::Dime;
    let result_dime = describe_state_quarter(dime);
    println!("Dime Status (expected None): {:?}", result_dime);
    println!("\n");
}


// =============================================================================
// MAIN ENTRY POINT
// =============================================================================

fn main() {
    demo_ip_addresses();
    demo_message_enum();
    demo_match_flow();
    demo_option();
    demo_concise_control_flow();
}



/*
================================================================================
                         SUMMARY & BEST PRACTICES
================================================================================

1. WHEN SHOULD YOU USE ENUMS?
-----------------------------
Use an Enum when your data represents a single choice out of a fixed set of 
possibilities (an "OR" relationship).

Examples:
- A state machine: State::Loading | State::Success(Data) | State::Error(Err)
- Network messages: Message::Ping | Message::Text(String) | Message::Disconnect
- Optional values: Option<T> -> Some(T) | None
- Error handling: Result<T, E> -> Ok(T) | Err(E)


2. WHEN SHOULD YOU USE STRUCTS INSTEAD?
---------------------------------------
Use a Struct when your data must hold multiple pieces of information at the 
same time (an "AND" relationship).

Examples:
- User profile: Name AND Email AND Age
- 2D Point: X coordinate AND Y coordinate


3. WHY ARE RUST ENUMS INNOVATIVE? (Tagged Unions / Sum Types)
------------------------------------------------------------
In traditional OOP languages (like Java, C++, C#), enums are usually just 
named integers (e.g., RED = 0, GREEN = 1).

In Rust, Enums are "Algebraic Data Types" (ADT):
- Each variant can carry different types and amounts of data.
- They completely replace fragile patterns like `null` pointers or class 
  inheritance hierarchies.
- The compiler strictly enforces safety at compile time.


4. PROS AND CONS OF RUST ENUMS
------------------------------

PROS:
+ Type Safety & Null Safety:
  Rust forces you to handle missing data (`Option`) and errors (`Result`) 
  explicitly. Null pointer crashes are prevented at compile time.

+ Exhaustive Pattern Matching:
  If you add a new variant to an enum, the Rust compiler will mark every 
  `match` block in your project as an error until you handle the new case. 
  This prevents hidden bugs when refactoring.

+ Flexible Payloads:
  One variant can be empty (`Quit`), another can hold numbers (`ChangeColor`), 
  and another can hold complex structs or strings (`Write`).

+ Memory Efficiency:
  An enum instance only uses enough memory to hold its largest variant plus a 
  tiny tag (1 byte) to identify which variant it currently is.

CONS:
- Verbose Unwrapping:
  To access data inside a variant, you must use pattern matching (`match`, 
  `if let`, or `let...else`), which can add boilerplate code.

- Memory Overhead of Large Variants:
  If one variant is very large (e.g., holds a 1MB array) and all other variants 
  are tiny (1 byte), every enum instance will allocate memory based on the 
  largest variant. (Fix: Wrap the large payload in `Box<T>`).

- Open Extension Difficulty:
  Unlike class inheritance, adding a new variant to a public library enum 
  can break downstream code because all existing `match` statements will 
  require updating (unless marked with `#[non_exhaustive]`).
================================================================================
*/