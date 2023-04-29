

fn main() {
    let v = vec![1, 2, 3, 4];
    let a: Vec<_> = v.iter().filter(|x| *x % 2 == 0).map(|x| x * 2).collect();
    let b: Vec<_> = v.iter().map(|x| x * 2).filter(|x| x % 2 == 0).collect();
    println!("{} {}", a[0], b[0]);
}
// a = [4, 8]
// b = [2, 4, 6, 8]
// output: 4 2