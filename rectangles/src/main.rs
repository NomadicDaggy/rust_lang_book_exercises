#[derive(Debug)] // This is called an annotation
struct Rectangle {
    width: u32,
    height: u32,
}
// To define a struct method, you have to put it inside a impl block.
// It's main purpose is to organize all struct methods in one place.
// ^ Does not make sense though, because multiple impl blocks for a
// single struct are allowed.
impl Rectangle {
    // If you omit self in method definition, it becomes an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        // and functions have to have (&)(mut) self as 1st arg
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
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
    // Associated functions can be accessed by ::
    let square = Rectangle::square(20);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold square? {}", rect1.can_hold(&square));
}
