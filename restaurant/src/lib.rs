mod front_of_house {  // This is a module
    // Making the module public, does not make its contents public
    pub mod hosting {  // Modules can be nested
        // And modules can have definitions of a bunch of things
        // like functions
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super is like ".."- you climb a level up
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    front_of_house::hosting::add_to_waitlist();
}