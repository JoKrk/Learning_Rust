use std::fs::File;
use std::io;
use std::fs;

fn main() {

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(_error) => panic!("Problem with opening text file"),
    // };

    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem with creating the file {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem with opening the file {:?}", other_error)
    //         }
    //     }
    // };

    // let _f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem with creating the file {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem with opening the file {:?}", error);
    //     }
    // });

    let _f = File::open("hello.txt").expect("Failed to open text file!");

    let _ = read_username_from_file();
}


// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}