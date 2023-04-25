// CHAPTER 8.1

fn main() {
    
    // Different ways to create a new Vector
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    // Reading elements of Vector
    // This one crashes if there is no third element
    let third: &i32 = &v[2];
    println!("Third element of vector is {}", third);

    // This one handles cases where there is no third element
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third element of vector is {}", third),
        None => println!("There is no third element"),
    }

    // Iterating over elements of a Vector (Immutable ref)
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        print!("{} ", n_plus_one);
    }
    
    // Iterating over elements of a Vector (Mutable ref)
    let mut v = vec![1, 2, 3, 4, 5];
    for n_ref in &mut v {
        *n_ref += 50;
    }

    for n_ref in &v {
        // n_ref has type &i32
        print!("{} ", *n_ref);
    }
    
    // Safely using iterators
    // More on Chapter 13.2
    // In general, when using iterators over Vectors,
    // We can iterate to read/modify CURRENT elements
    // but DO NOT add/remove elements with iteration

    // Two ways to iterate:
    // 1) Using references to each element
    // 2) Using a range to return references to each elem


}

