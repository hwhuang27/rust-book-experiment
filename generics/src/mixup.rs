struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// X1, Y1 defined here in impl to go with the struct definition
impl<X1, Y1> Point<X1, Y1> {
    // X2, Y2 defined in this method because they are only relevant to the method
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}