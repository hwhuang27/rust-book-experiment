
fn to_pig_latin(s: &mut String) -> &String {
    let first_char = s.remove(0);

    match first_char {
        'a' |'e' | 'i' | 'o' | 'u' => {
            s.insert(0, first_char);
            s.push('-');
            s.push_str("hay");
        }
        _ => {
            s.push('-');
            s.push(first_char);
            s.push_str("ay");
        }
    };
    s
}

fn main() {
    let mut my_string = "orange".to_string();

    println!("original string = {}", my_string);   
    println!("pig latin string = {}", to_pig_latin(&mut my_string));   
}