
// Struct examples:
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


// area takes the struct as a reference, so it does not take ownership of the struct. 
//This means that the struct can still be used after the function call. 
// If we had taken ownership of the struct, we would not be able to use it after the function call. This is because Rust has a strict ownership model that prevents

// create struct of rectangle and calculate area of rectangle using a method
// is like a class with methods cause area can only be called on rectangle struct it needs a rectangle as parameter