use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {

    // let b = Box::new(5);
    
    // println!("b = {}", b);
  
    //----------------------------

    // let list = Cons(1, Box::new(Cons(2,Box::new(Cons(3, Box::new(Nil))))));

    //-------------------------

    // let x = 5;
    // let y = &x;

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // ----------------

    // let x = 5;
    // let y = Box::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    //-----------------

    // let x = 5;
    // let y = MyBox::new(x);

    //--------------------

    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);

    //-----------------

    let c = CustomerSmartPointer {
        data: String::from("my stuff")
    };

    }
    



struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}!'", self.data);
    }
}



fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new (x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
Cons(i32, Box<List>),
Nil,
}