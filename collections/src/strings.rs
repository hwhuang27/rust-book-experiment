// CHAPTER 8.2

fn main() {
    // Create a new String
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // Update a string
    let mut s = String::from("foo");
    s.push_str("bar");
    
    assert_eq!("foobar", s);
    println!("{}", s);

    // Concatenation with the + Operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // Using the format! macro to concatenate
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Methods for iterating thru Strings

    // For individual Unicode scalar values, use .chars()
    for c in "Зд".chars() {
        println!("{}", c);
    }
    // To return bytes, use .bytes()
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}