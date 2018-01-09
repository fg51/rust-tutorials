//! this is module-level document
//! `adder` crate provides fuction
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// this function-level document
///
/// # Example
///
/// ```
/// use adder::add_two;
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_works_panic() {
        assert!(false);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(4, super::add_two(2));
    }
}
