fn main() {
    let mut dst = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let src = &[String::from("fourteen")];
    add_big_strings(&mut dst, src);
    
    println!("big strings");

}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}