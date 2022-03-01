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

// it is possible to bring a path into scope so you don't 
// have to repeat yourself all the time
/*
    Use keyword
    scopes check privacy as well
*/

use crate::front_of_house::hosting;
// can bring function into scope as well
// use crate::front_of_house::hosting::add_to_waitlist();
// using relative instead of absolute
// use self::front_of_house::hosting;
pub fn eat_using_scope() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/*
    When bringing in structs, enums, and other items
    use the full path, unless there are two items with the same
    name
*/
use std::collections::HashMap;
use std::fmt;
use std::io;

// fn func_1() -> fmt::Result {

// }

// fn func_2() -> io::Result {

// }

pub fn use_hashmap() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// you can also provide new names using the as keyword
// similar to JS, but use as and use keywords...
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// re export names with pub use
// scopes are private by default
// but you can make them public

// external code can now call and use this module
/*
Useful when the structure of your code is not how 
users will call it when importing it.
*/
pub use crate::front_of_house::serving;

/*

*/