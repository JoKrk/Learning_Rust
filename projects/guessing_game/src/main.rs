use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {


    let secret_num = rand::thread_rng().gen_range(0..101);

    println!("This is a guess the number game");
    
    //println!("Your secret number is {}", secret_num);

    loop
    {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed {}", guess);

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
                println!("Correct, you win!");
                break;
            }
        }
    }

  

}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Gues value must be between 1 and 100, got {}", value);
        }
    
        Guess {value}
    }
    
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

