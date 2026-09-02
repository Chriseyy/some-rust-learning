

// Methods are functions that are defined within the context of a struct, enum, or trait object. 
// They are associated with a particular type and can access the data of that type. In Rust, methods are defined using the `impl` keyword.

struct Rectangle {
    width: u32,
    height: u32,
}

// this is an implementation block for the Rectangle struct. 
// It defines methods that can be called on instances of the Rectangle struct.
// The "impl" keyword is used to define an implementation block for a struct, enum, or trait object.
//it is like fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}


// good to know stuff:
// p1.distance(&p2);
// (&p1).distance(&p2);

// The first line calls the distance method on p1, passing a reference to p2 as an argument.
// The second line does the same thing, but it explicitly takes a reference to p1 before calling the method. 
// In Rust, the first line is more idiomatic and preferred, as it is more concise and easier to read.
// Here’s how it works: When you call a method with object.something(), Rust automatically adds in &, &mut, or * so that object matches the signature of the method. In other words, the following are the same:



// --------

fn methonds_more_parameters() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    // // would be like a function:
    // fn can_hold(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    //     rect1.width > rect2.width && rect1.height > rect2.height
    // }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // does rect2 fit in rect1? yes, because rect1 is bigger than rect2 in both width and height
    // does rect3 fit in rect1? no, because rect3 is wider than rect1, even though it is shorter in height. The can_hold method checks both dimensions to determine if
}


// --------

fn associated_functions() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    // This makes a new rectangle with the same width and height, which is a square. 
    //The square function is an associated function because it is defined within the context of the Rectangle struct, but it does not take a &self parameter. 
    // This means that it can be called without an instance of the Rectangle struct, and it returns a new instance of the Rectangle struct.

    let sq = Rectangle::square(3);  // creates an rectangel with width and height of 3, which is a square. The :: syntax is used to call the associated function on the Rectangle struct, rather than on an instance of the struct.
    println!("Square: {} x {}", sq.width, sq.height);
}


// --------

fn multi_impl_blocks() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()  // this calls the area method on the rect1 instance of the Rectangle struct. The &self parameter is automatically passed to the method, so we don't need to pass it explicitly.
    );

    if rect1.width() {
        println!("The rectangle has a non-zero width of {} pixels.", rect1.width);
    }

    methonds_more_parameters();
    associated_functions();
    multi_impl_blocks();
}
