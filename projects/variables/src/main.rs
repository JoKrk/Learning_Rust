use std::io;

fn main() {

    // let x = 43 % 5;

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("The value of the tuple.1 is {}", tup.0);

    // let (x, y, z) = tup;
    // println!("The value of the tuple.2 is {}", y);

    // let arr = [1,2,3,4,5];

    // let a: [i32; 5] = [1,2,3,4,5];

    // let a3 = [3;5];

    let arr = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {} is: {}", index, element);

}
