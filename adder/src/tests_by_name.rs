pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // ignores this test
    // cargo test -- --ignored 
    // -- runs all ignored tests
    #[ignore]
    fn expensive_test() {

    }

    // cargo test one_hundred
    // -- runs only the 3rd test
    
    // cargo test add 
    // -- runs first and second test, filters out only tests that contain "add"
}