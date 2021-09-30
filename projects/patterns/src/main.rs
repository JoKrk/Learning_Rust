fn main() {


    //EXAMPLES OF IF LET EXPRESSIONS

    // let favourite_colour: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();

    // if let Some(colour) = favourite_colour {
    //     println!("Using your favourite colour, {}, as the background", colour);
    // } else if is_tuesday {
    //     println!("Tuesday is a green day!");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background col");
    //     } else {
    //         println!("Using orange as the background col");
    //     }
    // } else {
    //     println!("Using blue as the background col");
    // }

    //---------------------------------------

    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("Got 50!"),
    //     Some(y) => println!("Matched, y = {:?}", y),
    //     _ => println!("Default case, x = {:?}", x)
    // }

    // println!("at the end: x = {:?}, y = {:?}", x, y);

    //-------------------------------------------

    // let x = 1;

    // match x {
    //     1 | 2 => println!("one or two"), //or pattern match
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    //------------------------------------------

    // let x = 5;

    // match x {
    //     1..=5 => println!("one through five"), //match range of value with ':='
    //     _ => println!("something else"),
    // }

    //------------------------------------------

    // fn main() {
    //     let p = Point { x: 0, y: 7 };
    
    //     let Point { x: a, y: b } = p; //using patterns to deconstruct the point into variables a & b
    //     assert_eq!(0, a);
    //     assert_eq!(7, b);
    // }


    // let p = Point { x: 0, y: 7 };

    // match p {
    //     Point { x, y: 0 } => println!("On the x axis at {}", x), 
    //     Point { x: 0, y } => println!("On the y axis at {}", y),  //deconstructing while matching
    //     Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    // }
    //-----------------------------------------------


    // let msg = Message::Move {x: 5, y: 3};

    // match msg {
    //     Message::Quit => {
    //         println!("The quit variant has no data");
    //     }
    //     Message::Move {x, y} => {
    //         println!(
    //             "Move in the x direction {} and in the y direction {}",
    //             x, y
    //         );
    //     }
    //     Message::Write(text) => println!("Text msg: {}", text),
    //     Message::ChangeColor(r, g, b) => println!(
    //         "Change the color to red {}, green {}, blue {}",
    //         r ,g, b
    //     ),
    // }

    //----------------------------------------------


    //deconstruction two levels deep
   
    //     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    
    //     match msg {
    //         Message::ChangeColor(Color::Rgb(r, g, b)) => println!( 
    //             "Change the color to red {}, green {}, and blue {}",
    //             r, g, b
    //         ),
    //         Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
    //             "Change the color to hue {}, saturation {}, and value {}",
    //             h, s, v
    //         ),
    //         _ => (),
    //     }

    //----------------------------------------------


    //Using _ to handle matching
    // let mut setting_value = Some(5);
    // let new_setting_value = Some(10);

    // match (setting_value, new_setting_value) {
    //     (Some(_), Some(_)) => {
    //         println!("Can't overwrite an existing customized value");
    //     }
    //     _ => {
    //         setting_value = new_setting_value;
    //     }
    // }

    // println!("setting is {:?}", setting_value);

    //---------------------------------------------------


        //@ allows us to capture the variable that matched in the arm

    //     let msg = Message::Hello {id: 5};

    //     match msg {
    //         Message::Hello {
    //             id: id_variable @ 3..=7,
    //         } => println!("Found an id in range: {}", id_variable),
    //         Message::Hello { id: 10..=12} => {
    //             println!("Found an id in another range")
    //         }
    //         Message::Hello {id} => println!("Found some other i: {}", id)
    //     }
    // }

    
}

// enum Message {
//     Hello {id: i32},
// }

    // enum Color {
    //     Rgb(i32, i32, i32),
    //     Hsv(i32, i32, i32),
    // }
    
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(Color),
    // }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// struct Point {
//     x: i32,
//     y: i32,
// }