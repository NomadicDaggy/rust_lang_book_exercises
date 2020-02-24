#[derive(Debug)]  // this is an annotation
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height : 50 };

    // Print Rectangle instance, by deriving Debug trait in line 1
    // Can also pretty print by using {:#?} instead of {:?}
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// Take a reference to Rectangle, so that main can keep using it
// i.e. so that we don't have to explicitly pass it back
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
