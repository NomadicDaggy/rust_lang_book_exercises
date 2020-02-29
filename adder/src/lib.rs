#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(2 + 2, 4);
    }

    // Failing test
    // you have to make/let it panic if smth is wrong
    #[test]
    fn another() {
        panic!("panicerino");
    }
}
