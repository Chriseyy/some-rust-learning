

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


fn main() {
    refernce_string();
    mutable_references_1();
}