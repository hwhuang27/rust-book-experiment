
// Where to use generic types?
//  Structs | Enums | Functions | Methods

// multiple generic types with struct
struct Points<T, U> {
    x: T,
    y: U,
}

// in method definitions
struct Point<T, U> {
    x: T,
    y: T,
}
// implementing generic function
impl<T> Point<T> {
    fn x(&self) -> T {
        &self.x
    }
}
// impl block that only applies to a struct with a particular concrete type 
// for the generic parameter T
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// the Option enum (one generic type)
enum Option<T> {
    Some(T),
    None,
}

// the Result enum (two generic types)
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let both_integer = Points { x: 5, y: 10 };
    let both_float = Points { x: 1.0, y: 4.0 };
    let integer_and_float = Points { x: 5, y: 4.0 };
}