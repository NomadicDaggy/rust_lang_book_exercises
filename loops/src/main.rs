fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break works sorta like return here
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
