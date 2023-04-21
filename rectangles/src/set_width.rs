#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main(){
    let mut rect = Rectangle {
        width: 0,
        height: 0
    };
    rect.set_width(1);     // this is now ok

    let mut rect_ref = &mut rect;
    rect_ref.set_width(2);
    dbg!(&rect);
}
