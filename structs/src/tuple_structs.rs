// Tuple structs
// When you want to give a name to a tuple
// and have a less verbose format vs. normal structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like Structs w/o any fields
struct AlwaysEqual;

fn main() {
    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    let subject = AlwaysEqual;

}