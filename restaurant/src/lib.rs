// We can define this in another file with the same name
mod front_of_house;

mod back_of_house {
    // Making a struct public, doesn't make its fields public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // This is how you define struct methods
    impl Breakfast {
        // This has to be public, otherwise we can't construct the struct
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // If you make an enum public, all its variants become public too
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// By putting pub here, hosting becomes globally public
// It's called re-exporting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// Nested paths
// brings in std::io and std::io::Write
use std::io::{self, Write};

// "*" is the glob operator
// you can use it to bring all the public items in its scope
use std::collections::*;
