// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

//-------------------

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// fn serve_order() {}

//-------------------------------

// pub fn eat_at_resturant() {
//     //Abs path
//     crate::front_of_house::hosting::add_to_waitlist();

//     //rel path
//     front_of_house::hosting::add_to_waitlist();
// }

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

//-------------------------------

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches")
//             }
//         }
//     }
// }

// pub fn eat_at_resturant() {
//     //order a breakfast in summer with rye
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     //Change the bread
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // meal.seasonal_fruit = String::from("strawberries");
// }

//-----------------------------------

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// pub fn eat_at_resturant() {
//     hosting::add_to_waitlist();
// }

//----------------------------

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

//-----------------------------

// use std::{cmp::Ordering, io}; // use a nested path to bring multiple items with same prefix into scope
// use std::io::{self, Write}; 
// use std::collections::*; // bring all public items into scope with *

//-----------------------------

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

