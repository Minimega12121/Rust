pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
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
//Specific implementation for newsletter
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        //Default implementation
        format!("Read more... {}", self.summarize_author())
    }
}
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let a = NewsArticle {
        author: String::from("Archit"),
        headline: String::from("Hello World"),
        content: String::from("Hello World"),
    };
    let b = Tweet {
        username: String::from("Arhcit"),
        content: String::from("hello"),
        reply: false,
        retweet: false,
    };
    println!("Tweet sumamry: {}", b.summarize());
    println!("NewsArticle sumamry: {}", a.summarize());
    notify(&a);
}
