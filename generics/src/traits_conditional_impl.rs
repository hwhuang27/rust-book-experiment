use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let pair_int = Pair::new(3, 4);
    println!("{}, {}", pair_int.x, pair_int.y);
    pair_int.cmp_display();

    let pair_str = Pair::new("hello".to_string(), "world".to_string());
    println!("{}, {}", pair_str.x, pair_str.y);
    pair_str.cmp_display();

}