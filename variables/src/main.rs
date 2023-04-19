//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    /*

    // mutability and constants
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is {x}");
    }

    println!("The value of x is {x}");    
    
    let spaces = "      ";
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{spaces}");

    // scalar types (primitives in other langs)
    // integers, floats, characters, bools
    let c: char = '😻';
    println!("{c}");
     */ 

    // compound types (tuples & arrays)
    // tuples
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let six_point_four = tup.1;

    println!("{six_point_four}");
}