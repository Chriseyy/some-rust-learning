
// What is a Scruct in Rust?
// A struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group. 
// Structs are similar to classes in other programming languages, but they do not have methods or inheritance. 
// They are used to create complex data types that can hold multiple pieces of data.

use std::iter::Cloned;

fn example_struct() {
    // Define a struct named `Person`
    struct Person {
        name: String,
        age: u32,  // The `name` field is of type `String`, and the `age` field is of type `u32` u32 was int but cant be negative
    }

    // Create an instance of the `Person` struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Access the fields of the struct
    println!("Name: {}, Age: {}", person1.name, person1.age);
}

// Doesnt need to be called, but is here for demonstration purposes
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,  // these fields are set
        sign_in_count: 1,
    }
}


fn struct_update_example() {
    // this makes a new user based on the first one, but with a different username. The rest of the fields are copied from user1.
    let user1 = build_user(String::from("mutable@fdsafg.com"), String::from("mutable"));
    let user2 = User {
        email: user1.email.clone(),  // without coping the email, it would be moved from user1 to user2, and user1 would no longer be valid. This is because String is not Copy, so it cannot be copied. Instead, we can use the struct update syntax to copy the remaining fields from user1 to user2.
        username: String::from("user2"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("User1: {}, Email: {}, Active: {}, Sign-in Count: {}", user1.username, user1.email, user1.active, user1.sign_in_count);
    println!("User2: {}, Email: {}, Active: {}, Sign-in Count: {}", user2.username, user2.email, user2.active, user2.sign_in_count);


    // other way to do this is with the struct update syntax, which is more concise and easier to read. It copies the remaining fields from user1 to user2.
    let user3 = User {
        username: String::from("user3"),
        ..user1  // this copies the remaining fields from user1 to user3
    };
    println!("User3: {}, Email: {}, Active: {}, Sign-in Count: {}", user3.username, user3.email, user3.active, user3.sign_in_count);
}


fn types_with_tuple_structs() {
    struct Color(i32, i32, i32);  // This is a tuple struct, which is a struct with unnamed fields. It is similar to a tuple, but it has a name and can be used to create new types.
    struct Point(i32, i32, i32);  // This is another tuple struct, which is a struct with unnamed fields. It is similar to a tuple, but it has a name and can be used to create new types.

    let black = Color(0, 0, 0);  // This creates an instance of the Color struct with the values (0, 0, 0).
    let origin = Point(0, 0, 0);  // This creates an instance of the Point struct with the values (0, 0, 0).

}

fn empty_structs() {
    struct AlwaysEqual;  // This is an empty struct, which is a struct with no fields. It is used to create a type that has no data associated with it.

    let subject = AlwaysEqual;  // This creates an instance of the AlwaysEqual struct.
    // useful for what ? 
    // Empty structs can be useful when you want to implement a trait for a type but don't need to store any data. 
    // For example, you might have a trait that defines some behavior, and you want to implement that trait for a type that doesn't need to store any data. 
    // In that case, you can use an empty struct as the type.
}

// fn ownership_of_structs() {
//     struct User {
//         active: bool,
//         username: &str,  // why have &str here instead of String? Because we want to demonstrate ownership and borrowing. If we used String, we would have to clone the string when creating a new user, which would be inefficient. By using &str, we can borrow the string from the caller, which is more efficient.
//         email: &str,
//         sign_in_count: u64,
//     }

//     // This will casue an error because the struct User has a lifetime parameter, which means that the lifetime of the references in the struct must be specified. In this case, we can specify the lifetime of the references as 'static, which means that the references will live for the entire duration of the program. However, this is not ideal because it means that the references will never be dropped, which can lead to memory leaks. Instead, we can use String instead of &str, which will allow us to own the data and avoid lifetime issues.
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "some@email.cmo",
//         sign_in_count: 1,
//     };

// }


fn main() {
    example_struct();
    let email=String::from("heheh@fdsafg.com");
    let username=String::from("heheh");
    let user = build_user(email, username);
    println!("User: {}, Email: {}, Active: {}, Sign-in Count: {}", user.username, user.email, user.active, user.sign_in_count);
    // Intresting ist Strings in user cant be changed after creation, but the struct itself can be mutable if declared as such.
    let mut mutable_user = build_user(String::from("mutable@fdsafg.com"), String::from("mutable"));
    println!("Mutable User: {}, Email: {}, Active: {}, Sign-in Count: {}", mutable_user.username, mutable_user.email, mutable_user.active, mutable_user.sign_in_count);
    mutable_user.email = String::from("mutable_user@email.com");
    println!("Updated Mutable User: {}, Email: {}, Active: {}, Sign-in Count: {}", mutable_user.username, mutable_user.email, mutable_user.active, mutable_user.sign_in_count);
    // This mutable_user is mutable, so we can change its fields after creation. However, the original user variable is not mutable, so we cannot change its fields after creation.

    struct_update_example();
    types_with_tuple_structs();
    empty_structs();
    // ownership_of_structs();
}
