
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    };
}

fn main() {
    let five = 5;
    let six = plus_one(five);
    println!("{}", );
    let none = plus_one(None);

}