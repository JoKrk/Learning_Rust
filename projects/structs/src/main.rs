fn main() {

    // let width1 = 30.0;
    // let height1 = 50.0;

    // println!("The area of the rect. is {} mm", area(width1, height1));

    // println!("Enter any key to exit");
    //------------------------------------------


    let rect = Rectangle {
        width: 30.0,
        height: 50.0
    };

    let rect2 = Rectangle::square(30.0);



    println!("can rect2 [{:?}] fit in rect1 [{:?}]? : {} ", rect2, rect, rect.can_hold(&rect2));

    let area_r = rect_area(&rect);
    let area_from_meth = rect.area();

    println!("Inputted rectangle: {:?}", rect);
    println!("The area of the rect. is {} mm", area_r);
    println!("The area of the rect. (version 2) is {} mm", area_from_meth);




    //--------------------------------------------
    // println!("Enter any key to exit");
    // let mut exit = String::new(); 
    // io::stdin()
    // .read_line(&mut exit) 
    // .expect("Failed to read line");
}

    


// fn area(width: f64, height: f64) -> f64 {
//     width * height
// }

fn rect_area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.height >= rect2.height && self.width >= rect2.width
    }
}

impl Rectangle {
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}