
// example that uses generic types, traits, and lifetimes
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = "this string";
    let b = "that string haha";
    let c: String = String::from("this is my announcement");

    let res = longest_with_an_announcement(&a, &b, c);
    println!("{}", res);
}