fn greeting() {
    let m1 = String::from("hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("{s}");


}

fn greet(g1: &String, g2: &String) {
    println!("{} {}", g1, g2);
}
