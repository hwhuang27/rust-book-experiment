struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };    
    
    let e = CustomSmartPointer {
        data: String::from("drop this early"),
    };
    drop(e);

    let mut s = String::new();
    drop(s);
    // (|_| ())(s);
    // s;
}