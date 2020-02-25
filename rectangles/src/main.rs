#[derive(Debug)]  // this is an annotation
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
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
