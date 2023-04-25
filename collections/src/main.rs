// CHAPTER 8.3 EXERCISES

use std::collections::HashMap;

fn main() {
    let mut v = vec![5, 7, 23, 47, 11, 78, 4, 2, 4, 5, 4];
    v.sort();

    let median = &v[v.len() / 2];
    println!("{:?}", v);
    println!("Median = {}", median);

    let mut hm = HashMap::new();
    for num in &v {
        let count = hm.entry(num).or_insert(0);
        *count += 1;
    }
    
    let mode = hm.iter().max_by_key(|&(k, v)| v);

    println!("Mode = {}", mode);
    

}


