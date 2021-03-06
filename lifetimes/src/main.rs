// Setting lifetimes doesn't change how long a value lives
// it just makes clear for the compiler how long each value
// should live, so it can reject the ones that misbehave.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This doesn't work, because both s1 and s2 have to be within scope
// of last use of result
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
