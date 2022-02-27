// modules control privacy of items

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
/*
    Modules can also hold definitions for structs, enums, constants, traits, or functions
    The below code creates a module tree

    Paths have two forms (absolute, relative)
    use absolute paths...

    everything is private by default

*/
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    // attributes of structs are all private by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // if an enum is public, then all of it's variants are also public
    pub enum Appetizer {
        Soup,
        Salad
    }

    fn fix_incorrect_order() {
        cook_order();
        crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}


// this code can now be called by anyone that has imported our crate
pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();

    // // relative
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please!", meal.toast);

    let order_1 = back_of_house::Appetizer::Salad;
    let order_2 = back_of_house::Appetizer::Soup;

}