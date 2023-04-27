use std::fs;
use std::io;

// shortest version
// opens file, create new string, read contents from file, 
// copies to string, & returns it
fn read_username_from_file() -> Result<String, io::Error> {

    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let username_result = read_username_from_file();

    match username_result {
        Ok(str) => println!("{str}"),
        Err(err) => panic!("returned error: {}", err),
    }
}

