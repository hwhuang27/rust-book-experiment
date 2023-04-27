pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess has to be between 1 and 100!, got {}", value);
        }

        Guess { value } 
    }

    pub fn value(&self) -> i32{
        self.value
    }

}
fn main() {
    // implementing a Guess struct will force users to give a correct value before continuing
    let my_guess = Guess::new(99);
    println!("guess: {}", my_guess.value());
}

