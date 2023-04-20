fn some_strings() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    
    // method 1: use an immutable/shared ref.
    let s_ref: &String = &v[0];
    println!("{s_ref}");

    // method 2: clone to get ownership of the String
    let mut s_clone: String = v[0].clone();
    s_clone.push('!');
    println!("{s_clone}");

    // method 3: move string out of vector (vec::remove)
    let mut s_removed: String = v.remove(0);
    s_removed.push('!');
    println!("{s_removed}");
    
}
