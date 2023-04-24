enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main () {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(3.62),
    ];
    
    print_vector(&row);
}

fn print_vector(v: &Vec<SpreadsheetCell>) {
    for e in v {
        match e {
            SpreadsheetCell::Int(data) => println!("{}", data),
            SpreadsheetCell::Float(data) => println!("{}", data),
            SpreadsheetCell::Text(data) => println!("{}", data),
        }
    }
}