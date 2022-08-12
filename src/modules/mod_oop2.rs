use std::time::Instant;


#[derive(Debug)]
struct Post {
    content: Box<String>,
    review: bool,
    approved: bool,
}

impl Post {
    fn new(content: String) -> Self {
        Self {
            content: Box::new(content),
            review: false,
            approved: false,
        }
    }
}

trait Blog {
    fn update_content(&mut self, content: String);
    fn review(&mut self);
    fn approved(&mut self);
    fn show(&self) -> Post;
}

impl Blog for Post {
    fn update_content(&mut self, content: String) {
        self.content.push_str(&content)
    }
    fn review(&mut self) {
        self.review = true
    }
    fn approved(&mut self) {
            self.approved = true
    }
    fn show(&self) -> Post {
        Post {
            content: Box::clone(&self.content),
            review: self.review,
            approved: self.approved,
        }
    }
}


// blog object oriented program
pub fn main() {
    let s = String::from("starbucks coffee is delicious");
    let mut post = Post::new(s);
    post.update_content(String::from("I love coffee that buy starbucks coffee"));
    post.review();
    post.approved();
    let show_post = post.show();
    println!("{:#?}", show_post);

    elapsed_time();
}


fn elapsed_time() {
    let start = Instant::now();

    let mut count = 0;
    for i in 0..10 {
        count += i
    }

    print!("{} {:?}", count, start.elapsed());
}