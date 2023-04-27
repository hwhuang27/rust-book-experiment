use std::fs::File;
use std::io;
use std::io::Read;

// shorter version
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    let username_result = read_username_from_file();

    match username_result {
        Ok(str) => println!("{str}"),
        Err(err) => panic!("returned error: {}", err),
    }
}
