use std::future::Pending;

// pub struct AveragedCollection {
//     list: Vec<i32>,
//     average: f64
// }

// impl AveragedCollection {
//     pub fn add(&mut self, value: i32) {
//         self.list.push(value);
//         self.update_average();
//     }

//     pub fn remove(&mut self) -> Option<i32> {
//         let result = self.list.pop();
//         match result {
//             Some(value) => {
//                 self.update_average();
//                 Some(value)
//             }
//             None => None,
//         }
//     }
    
//     pub fn average(&self) -> f64 {
//         self.average
//     }
    
//     fn update_average(&mut self) {
//         let total: i32 = self.list.iter().sum();
//         self.average = total as f64 / self.list.len() as f64;
//     }
// }


// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Screen {
//     pub components:Vec<Box<dyn Draw>>
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self .components.iter() {
//             component.draw();
//         }
//     }
// }

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String
// }

// impl Draw for Button {
//     fn draw(&self) {
//         // code to draw
//     }
// }

//----------------------------------------

// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new()
//         }
//     }

//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }
// }



// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn reject(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, _post: &'a Post) -> &'a str {
//         ""
//     }

//     // fn add_text<'a>(&self, _post: &'a mut Post, _text: &str) {
//     // }

// } 

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     // fn add_text<'a>(&self, post: &'a mut Post, text: &str) {
//     //     &post.content.push_str(text);
//     // }
// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
        

//         Box::new(Published{})
//     }
    
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Draft {})
//     }


// }

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }


//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content()
//     }
    
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }


//------------------------------------

pub struct Post {
    content: String
}

pub struct DraftPost {
    content: String
}

impl Post {
    pub fn new () -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}


