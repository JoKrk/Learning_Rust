fn main() {

    // let s1 = gives_ownership();

    // let s2 = String::from("second ownership");

    // let s3 = gives_and_takes_back(s2);

    // println!("string length: {}", calc_length_ref(&s3));


    // let mut r1 = String::from("hello");

    // {
    //     let r2 = &mut r1;
    // } // r2 goes out of scope here, so we can make a new reference with no problems.

    // let r3 = &mut r1;

    //---------------------------

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // r1 and r2 are no longer used after this point so r3 can be a mut ref

    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    //---------------------

    //slices
    // let s = String::from("hello");
    // let slice = &s[0..2]; //these are equal, no need for starting 0
    // let slice = &s[..2]; //these are equal, no need for starting 0

    //------------------

    // let mut s = String::from("hello world");

    // let word = return_first_word(&s);

    // s.clear(); // error!

    // println!("the first word is: {}", word);

    //-------------------

    let first = return_first_word("test word");
    println!("{}", first);
}


fn return_first_word(sentence: &str) -> &str { //pass in ref string
    let bytes = sentence.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[..i];
        }
    }

    &sentence[..]
}

// fn gives_ownership() -> String {
//     let string = String::from("ownership");
//     string
// }

// fn gives_and_takes_back(a_string: String) -> String {
//         a_string
// }

// fn calc_length_ref(ref_string: &String) -> usize { //ref_string is a reference to s3 delcared in the main func
//     let length = ref_string.len();
//     length
// }