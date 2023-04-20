mod temps;

fn main() {
    // convert_temp(mut temperature: f64, mut is_celsius: bool)
    temps::convert_temp(34.0, true);
    
    let a = Box::new([0; 10_000]);

    println!("{}", a[0]);
}

