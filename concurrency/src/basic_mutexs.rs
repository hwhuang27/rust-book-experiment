use std::sync::Mutex;

// Mutex is similar to RefCell<T> but in concurrent situations (moving between threads)
fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}