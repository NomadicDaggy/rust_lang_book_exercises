#[derive(Debug)]  // This is called an annotation
struct Rectangle {
    width: u32,
    height: u32,
}
// To define a struct method, you have to put it inside a impl block.
// It's main purpose is to organize all struct methods in one place.
impl Rectangle {
    fn area(&self) -> u32 {  // and functions have to have (&)(mut) self as 1st arg
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height : 50 };

    // Print Rectangle instance, by deriving Debug trait in line 1
    // Can also pretty print by using {:#?} instead of {:?}
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
