pub fn add_two(a: i32) -> i32 {
    println!("I got the value {}", a);
    a + 2
}

/*
 *  Run `cargo test -- --test-threads=1` to run tests consecutively
 *  Run `cargo test -- --show-output` to show `println!` output
 *  Run `cargo test two` to run tests with names that match "two"
 *  Run `cargo test -- --ignored` to run only ignored tests
 */
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

    // ignoring the test
    #[test]
    #[ignore]
    fn add_four_and_two() {
        assert_eq!(6, add_two(4));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
