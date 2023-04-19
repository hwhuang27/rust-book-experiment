const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // MUTABILITY & CONSTANTS
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // SHADOWING
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

    // SCALAR TYPES (AKA PRIMITIVES)
    // INTEGERS, FLOATS, CHARS, BOOLS
    let c: char = '😻';
    println!("{c}");

    // COMPOUND TYPES
    // TUPLES
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("[{}, {}, {}]", x, y, z);
    println!("[{}, {}, {}]", tup.0, tup.1, tup.2);

    // ARRAYS
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    
    let first = a[0];
    let second = b[1];

    let third = first + second;
    println!("value of third is: {third}");

}

