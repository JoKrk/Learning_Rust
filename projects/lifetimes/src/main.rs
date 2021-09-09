// use std::fmt::Display;

fn main() {


    // let r;
    // let x = 5;
    // r = &x;

    // println!("r: {}", r);

    //------------------------------

    // let string1 = String::from("abcd");
    // let string2 = String::from("xyz");

    // let result = longest(string1.as_str(), string2.as_str());
    // println!("The longest string is {}", result);

    //-------------------------------------

    // let string1 = String::from("long string is long");
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is '{}'", result);
    // }

    // //---------------------------------------

    // //lifetime annotations in structs
    // let novel = String::from("Call me Ismael. Some years ago...");
    // let first_sentence = novel.split('a').next().expect("Could not find a '.'");
    // let i = ImportantExcerpt {
    //     part: first_sentence,
    // };


    //----------------------------------------------

    // let s: &'static str = "I have a static lifetime.";

    //--------------------------------------------



}

// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T
// ) -> &'a str
// where
//     T:Display,
//     {
//         println!("Announcement! {}", ann);
//         if x.len() > y.len() {
//             x
//         } else {
//             y
//         }
//     }



// //-----------------------------------------
// //lifetime annotations in structs
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {

//     fn level(&self) -> i32 {
//         3
//     }

//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention: {}", announcement);
//         self.part
//     }
// }


//-----------------------------------------------------

//compiler can determine what lifetime parameters are needed if it follows the 3 rules
//first rule:
//each parameter that is a reference gets it own lifetime parameter
//second rule:
//if one parameter then output parameters will all get the same lifetime parameter
//third rule:
//ignore for now

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }


//-------------------------------------------------------------


// fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     }
//     else {
//         y
//     }
// }



