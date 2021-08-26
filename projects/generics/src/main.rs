fn main() {



    //  let number_list = vec![34, 50, 25,100, 65];


    //  let result = largest(&number_list);
    //  println!("The largest number is {}", result);
 
    //  let char_list = vec!['y', 'm', 'a', 'q'];
 
    //  let result = largest(&char_list);
    //  println!("The largest char is {}", result);
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    //println!("The largest number is {}", largest);

    //-----------------------------------------------------

    // let result = largest(&number_list);

    // println!("The largest number is {}", result);


    //----------------------------------

    // let integer = Point {x:5, y:10};
    // let float = Point {x:1, y:4};

    // println!("p.x = {}", integer.x());

    //-----------------------------------

    // let p1 = Point {x: 5, y:10.4};
    // let p2 = Point {x: "hello", y:'c'};

    // let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //---------------------------------------

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from(
    //         "of course, lorem ipsum"
    //     ),
    //     reply: false,
    //     retweet: false
    // };

    // println!("1 new tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };

    // println!("New article available! {}", article.summarize());


    //------------------------------------------
}


fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

}



//impl trait allows us to call the summarize trait method on any type that implement the Summary trait as a parameter

//shorthand syntax
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }

// long hand syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

//can require two traits at the same time
pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize())
}

pub trait Display {

    fn display(&self) -> String;
}

pub trait Debug {

}

pub trait Summary {

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }



}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// struct Point<T> {
//     x:T,
//     y:T,
// }

// struct Point<T, U> {
//         x:T,
//         y:U,
//     }
    

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V,W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }


// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];


//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }