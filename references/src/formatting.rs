fn formatting() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0];
    *x += 1;
    println!("{a:?}");

    let some_string: String = String::from("this_string");
    println!("{some_string:?}");
}
