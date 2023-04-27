use std::fs::File;
use std::io::ErrorKind;

fn main() {

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project.");
    //create_greeting_file();

    
}

fn create_greeting_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };
}
