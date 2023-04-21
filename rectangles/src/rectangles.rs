#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };    
    
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let sq = Rectangle::square(3);

    println!("{}", rect1.can_hold(&rect2));
    dbg!(&sq);
    // println!("the size of the rectangle is {} square pixels",
    //             rect1.area());
    
    // !dbg(&rect1);
}
