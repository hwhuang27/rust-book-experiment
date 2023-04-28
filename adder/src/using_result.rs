pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    // using Result<T, E> in tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // note: cannot use #[should_panic] on tests that use Result<T, E>
    // To assert that an operation returns an Err variant, 
    // don’t use the question mark operator on the Result<T, E> value. 
    // Instead, use assert!(value.is_err()).
}