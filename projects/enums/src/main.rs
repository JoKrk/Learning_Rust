fn main() {

    // let v4 = IpAddressKind::V4;
    // let v6 = IpAddressKind::V6;

    // // let home = IpAddress {
    // //     kind: v4,
    // //     address: String::from("127.0.0.1")
    // // };   

    // let home = IpAddressKind::V4(String::from("127.0.0.1"));

    // let null_number : Option<i32> = None;
    // let some_string = Some("string");

    //---------------------------------

    let some_number = Some(5);

    let x = 3;
    let y = some_number.unwrap();

    println!("{}", x + y);

    //----------------------------------

    let cents = value_in_cents(Coin::Dime);
    println!("The value of a dime in cents is {}", cents);

    //----------------------------------

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //-----------------------------

    let some_value = Some(0u8);
    if let Some(3) = some_value {
        println!("it's a three!")
    } else {
        println!("not a three!")
    }
    
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}


fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}


// fn route (ipKind: IpAddressKind) {
    
// }

// struct IpAddress {
//     kind: IpAddressKind,
//     address: String
// }

// enum IpAddressKind {
//     V4(String),
//     V6(String)
// }
