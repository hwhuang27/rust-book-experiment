pub fn convert_temp(mut temperature: f64, is_celsius: bool) {
    if is_celsius {
        temperature = temperature * (9.0/5.0) + 32.0;
       
    } else {
        temperature = (temperature - 32.0) * (5.0/9.0);
    }

    //let temperature = temperature as i32;     // type cast to signed 32 int
    println!("Your temperature is {} {}", temperature, if is_celsius {"C"} else {"F"});
}