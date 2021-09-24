use OOP::Post;

fn main() {


    let mut post = Post::new();

    post.add_text("I ate a salad today for lunch");

    let post = post.request_review();
    
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

}


// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         //placeholder
//     }
// }