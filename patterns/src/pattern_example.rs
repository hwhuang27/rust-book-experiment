fn main() {
    let mut v = vec![(1,2), (2,4)].into_iter();
    let mut sum = 0;
    while let Some(t) = v.next() {
        let (_, n) = t;
        sum += n;
    }

    println!("the sum is every 2nd number is {sum}");

}
