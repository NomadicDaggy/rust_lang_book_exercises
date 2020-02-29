#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

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

    // Put this here to bring code 1 lvl up from this
    // block into scope. Ie. Rectangle struct and impl.
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        //      v-----Usual negation works in Rust
        assert!(!smaller.can_hold(&larger));
    }
}