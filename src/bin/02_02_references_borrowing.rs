

// References_borrowing:
// a way to fix ownership problem 
// is to provide a refercen to the string value 

// A reference is like a pointer in that it’s an address we can follow to access the data stored at that address;
// ; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.


fn refernce_string() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}
// know you can call s1 without it vanishes the funciton doest "steal" the value 
//  note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String

fn calculate_length(s: &String) -> usize {
    s.len()
}// Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

// Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. 
// &s1 syntax lets us create a reference that refers to the value of s1 but does not own it


// refernces 
// fn change_string() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// this doesnt work cause Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
// to make it work | fn change(some_string: &mut String) {

// Mutable References
fn mutable_references_1() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// s is mutable now.. 
// First, we change s to be mut. Then, we create a mutable reference with &mut s where we call the change function and update the function signature to accept a mutable reference with some_string: &mut String
// This makes it very clear that the change function will mutate the value it borrows.

fn mutable_references_2() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        print!("{} ", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;   

    // print!("{} ", s;);
    // print!("{} ", r1);
    print!("{} ", r2);
}

// for my info we make s a String mutable in rust all stuff with mut is mutable, so we can change it.
// next we make a new variable r1 that is a mutable reference to s. this means it takes 
    // print!("{} ", s;);
    // print!("{} ", r1);
// this error cause ownership is now r2 and string s doesnt exist anymore, so we cant print it out.
// if we wnat to print out s we need to make r1 go out of scope first, so we can make a new reference with no problems.
// out of scope means that the variable is no longer valid and can no longer be used. r1 goes out of scope when the block ends, so we can make a new reference with no problems.
// r1 doesnt exist anymore, so we can make a new reference with no problems.


fn mutable_references_3() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{r1}, {r2}, and {r3}");
 
}

// this doesnt work cause we can have multiple immutable references or one mutable reference, but not both at the same time.
// r1 and r2 are immutable references to s, so we can have multiple immutable references to s. But we cannot have a mutable reference to s while we have immutable references to s. This is because if we had a mutable reference to s, we could change the value of s while we have immutable references to s, which would be a problem.

fn mutable_references_4() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1}, {r2}");
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}
// why does this work: cause r1 and r2 are no longer used after the println! macro, so we can make a mutable reference to s. This is because the compiler can see that r1 and r2 are no longer used after the println! macro, so it can allow us to make a mutable reference to s.
// above r1 and r2 where after r3 the print
// r1 and r2 only exist in the scope of the println! macro, so we can make a mutable reference to s after the println! macro.
// r1 and r2 are gone after the println r3 stay even after the print cause r3 is stil in scope because it is mutable reference to s, so we can make a mutable reference to s after the println! macro.

// --------------

// A String has two parts:
// 'Stack: A small struct storing (pointer to heap, length, capacity).
// Heap: The actual bytes of the text (e.g., ['h','e','l','l','o']).'
// A reference (&s or &mut s) is simply a pointer on the stack containing the memory address of the stack struct s.

// STACK                                            HEAP
// [ s: (ptr: 0x500, len: 5, cap: 5) ] ----------> [ 'h', 'e', 'l', 'l', 'o' ] (at 0x500)
//        ^
//        | (stores address of s)
// [ r1: 0x100 ]


// Borrowing does NOT move ownership: s remains the owner.

// When r1 dies: Only the tiny stack pointer r1 disappears. The heap memory stays alive because s owns it.

// When s dies (end of scope): Rust automatically deallocates the heap memory.

fn final_mutable_references() {
    // -------------------------------------------------------------
    // STEP 1: Ownership & Allocation
    // -------------------------------------------------------------
    // `s` is created on the Stack; "hello" lives on the Heap.
    // `s` is the SOLE OWNER.
    let mut s = String::from("hello"); 

    // -------------------------------------------------------------
    // STEP 2: Immutable Borrowing (&)
    // -------------------------------------------------------------
    // r1 and r2 are small pointer variables on the Stack.
    // Both hold the memory address of `s`.
    let r1 = &s; 
    let r2 = &s;

    // We READ through the references:
    println!("Step 2: Read via r1='{r1}' and r2='{r2}'");
    
    // --- LIFETIME CHECKPOINT (Non-Lexical Lifetimes - NLL) ---
    // This is the LAST time r1 and r2 are used.
    // Rust's compiler marks the borrow for r1 and r2 as ENDED right here.
    // Even though the variables r1 and r2 sit in this block, their *borrow* is dead.

    // -------------------------------------------------------------
    // STEP 3: Mutable Borrowing (&mut)
    // -------------------------------------------------------------
    // Now that no immutable references are active, we can borrow mutably.
    // `r3` gets exclusive access to `s`.
    let r3 = &mut s; 

    // While `r3` is active:
    // - You CANNOT read or write through `s` directly.
    // - You CAN modify the data through `r3`.
    r3.push_str(", world");
    println!("Step 3: Mutated via r3='{r3}'");

    // --- LIFETIME CHECKPOINT ---
    // This is the LAST use of `r3`. Its exclusive lock on `s` is released here.

    // -------------------------------------------------------------
    // STEP 4: Back to the Owner
    // -------------------------------------------------------------
    // All borrows are finished. We can use `s` directly again.
    println!("Step 4: Owner s is now='{s}'");

    // -------------------------------------------------------------
    // STEP 5: Explicit Block Scopes
    // -------------------------------------------------------------
    {
        let r4 = &mut s;
        r4.push_str("!");
        println!("Step 5: Inside inner scope='{r4}'");
    } // <-- r4 goes out of scope here. The variable itself is destroyed from the stack.

    println!("Step 5 (after block): '{s}'");

} // <-- Scope of `main` ends here.
  // `s` goes out of scope.
  // Rust automatically calls `drop` and frees the Heap buffer.


fn bad_example() {
    let mut s = String::from("hello");

    let r1 = &s;      // r1 borrows `s` as READ-ONLY.
    let r3 = &mut s;  // ERROR here: Cannot borrow `s` as mutable because `r1` is still alive!

    // Why did the error happen on the line above?
    // Because the compiler looks ahead and sees you use `r1` here:
    println!("r1 is: {r1}"); // `r1`'s lifetime extends to this line!
    
    r3.push_str(" world");
    println!("r3 is: {r3}");
}


fn good_example() {
    let mut s = String::from("hello");

    let r1 = &s;
    println!("r1 is: {r1}"); 
    // LAST USE of r1 is right here.
    // Rust automatically drops r1's read-lock immediately.

    // Now s is completely free:
    let r3 = &mut s; // OK: No other active references exist!
    r3.push_str(" world");
    println!("r3 is: {r3}");
}

// Why Rust Enforces This Rule
// If Rust allowed r1 and r3 to be used at the same time:

// r3.push_str(...) might resize the string on the heap, moving it to a new memory address.

// r1 would still point to the old memory address.

// Printing r1 would read freed or corrupt memory (a Dangling Pointer / Use-After-Free bug).

// Rust prevents this entirely at compile time by demanding that &mut references are completely isolated.


// ------------------------

// dangling references
// Dangling references are a problem in some programming languages. 
// A dangling reference points to a location in memory that may have been given to someone else. 
// Rust prevents dangling references by refusing to compile code that would let them occur.


fn dangling_reference() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!

// Here we create a reference to a String that is created inside the dangle function.
// When dangle returns, s goes out of scope and is dropped.
// The reference that we return will be pointing to deallocated memory, which is a dangling reference

// ============================================================================
// DANGLING REFERENCES (Why returning &String fails)
// ============================================================================
// 1. A reference (&s) is just a memory address, NOT the data itself.
// 2. When a function ends, all local variables (like `s`) are DROPPED (deallocated).
// 3. Returning `&s` returns an address to dead/freed memory ("Dangling Pointer").
// 4. Rust prevents this at compile time.
//
// THE FIX:
// Return the actual value (`String`), NOT the reference (`&String`).
// This MOVES ownership out of the function so the data stays alive.

// BROKEN: Returns an address to deleted memory (Compiler Error)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ERROR: `s` is dropped here, so &s points to nothing!
// }

// WORKING: Moves the whole String (ownership) out to the caller (Safe!)
fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership of `s` is moved out, data is NOT dropped.
}

fn main() {
    refernce_string();
    mutable_references_1();
    mutable_references_2();
    mutable_references_3();
    mutable_references_4();
    final_mutable_references();

    dangling_reference

}