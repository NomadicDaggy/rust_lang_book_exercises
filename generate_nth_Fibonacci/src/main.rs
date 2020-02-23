fn main() {
    /* Fibbonacci numbers go like:
    *   1, 1, 2, 3, 5, 8, 13, 21, 34, ...
    *    or
    *   n-2, n-1, (n-2) + (n-1), ...
    */
    let n = 20;
    println!("The {}th fibonacci number is: {}", n, nth_fibonacci(n));
}

fn nth_fibonacci(n: i32) -> i32 {
    // the "_f64" has to be here or it won't compile
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    // powf raises to a float power. Type casting is done with "as"
    return (phi.powf(n as f64) / 5.0_f64.sqrt()).round() as i32
}