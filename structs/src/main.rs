struct Name {
    first: String,
    last: String,
}

fn main() {
    let mut my_name = Name { first: "David".to_string(), last: "Huang".to_string()};
    my_name.first.push_str("QWE");
    let other_name = Name { last: "something".to_string(), ..my_name};
    //my_name.first.push_str("ASD");
    // // println!("{}", my_name.first);
}