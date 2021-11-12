// public function
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// private function
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// `#[cfg(test)]` tells the compiler that it's a unit test. Run the
// test code only when running `cargo test`, not `cargo build`.
#[cfg(test)]
mod tests {
    use super::*;

    // testing private function.
    // `tests` module has access to parent module.
    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 2), 4)
    }
}
