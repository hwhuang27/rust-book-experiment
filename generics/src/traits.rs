use std::fmt::Display;
use std::fmt::Debug;

// A Triat defines functionality a particular type has and can share with other types
// Similar to Interfaces from other languages, with some differences
pub trait Summary {
    fn summarize(&self) -> String{
        String::from("(Read more..)")   // default implementation
    }
}

// Implementing Summary trait on the NewsArticle and Tweet types
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// returns default implementation when called
impl Summary for NewsArticle {}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }


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
}

// Traits as parameters
// notify takes in any item that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// longer form as above, called Trait bounds
pub fn notify_tb<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// if we want item1 and item2 to have different types, both implementing Summary trait
pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {}

// force both parameters to have the same type 
pub fn notify_two_tb<T: Summary>(item1: &T, item2: &T) {}

// specify item with multiple traits with + operator
pub fn notify_multiple_traits(item: &(impl Summary + Display)) {}

// trait bound method
pub fn notify_multiple_traits_<T: Summary + Display>(item: &T) {}

// organize trait bounds when you have too many
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ 0 }

// return a type that implements a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);
}