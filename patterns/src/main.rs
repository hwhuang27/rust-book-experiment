fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, second, ..) => {
            println!("Some numbers: {} {}", first, second);
        },
    }
}