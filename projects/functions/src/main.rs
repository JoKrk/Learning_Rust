


fn main() {

    //println!("Main function");
    //another_function(5, 67);
    //expression_example();

    //let five = return_five();
    //println!("FIVE!: {}", five);

    //if_example();

    //loop_example();
    
    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    //     println!("the value is {}", element);
    // }
    
    let n = 10;
    let n_fib = get_fibonacci_number(n);
    println!("The nth [n = {}] fibonacci number is {}", n, n_fib);
}


fn get_fibonacci_number(n: i32) -> i32 {

    if n < 1 {
        return 0
    }
    
    let mut fib: i32 = 1;
    let mut fib_1: i32 = 1;
    let mut fib_2: i32 = 0;
    println!("fib_2 = {}, fib_1 = {}, fib = {}", fib_2, fib_1, fib);

    for _ in 1..n {
        fib = fib_1 + fib_2;
        println!("fib_2 = {}, fib_1 = {}, fib = {}", fib_2, fib_1, fib);
        fib_2 = fib_1;
        fib_1 = fib;
    }

    fib
}


// fn another_function(p1: i32, p2: i64) {
//     println!("Another function with parameter {}", p1);
//     println!("and second parameter {}", p2);

// }

// fn expression_example() {

//     let y = {
//         let x = 3;
//         x + 1 //no semi colon at the end as this block is an expression, not a statement. (expressions return a value, statements do not)
//     };

//     println!("the value of y is {}", y);
// }

// fn return_five() -> i32 {
//     5
// }

// fn if_example() {

//     // let number = 0;

//     // if number < 5  && number > 0{
//     //     println!("condition true!");
//     // } else if number == 0 {
//     //     println!("number is zero")
//     // } else {
//     //     println!("condition false!");
//     // }


//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {}", number);
// }


// fn loop_example()
// {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }
