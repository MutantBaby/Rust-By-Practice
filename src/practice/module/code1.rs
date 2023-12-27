pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
        pub fn complain() {}
    }
}

mod back_of_house {
    // use crate::front_of_house;

    fn fix_incorrect_order() {
        cook_order();

        // front_of_house::serving::serve_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::seat_at_table();
}
