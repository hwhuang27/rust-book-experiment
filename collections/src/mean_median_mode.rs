// CHAPTER 8.3 EXERCISES

use std::collections::HashMap;

fn average(arr: &[i32]) -> f32 {
    let sum: i32 = arr.iter().sum();
    sum as f32 / arr.len() as f32
}

fn median(arr: &mut [i32]) -> i32 {
    arr.sort();
    let mid = arr.len() / 2;
    arr[mid]
}

fn mode(arr: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &num in arr{
        let count = occurrences.entry(num).or_insert(0);
        *count += 1;
    }
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute mode of empty array")
}
fn main() {
    let mut arr = [1, 2, 22, 44, 11, 33, 4, 2, 2, 4];
    println!("The average is {}", average(&arr));
    println!("The median is {}", median(&mut arr));
    println!("The mode is {}", mode(&arr));
}


