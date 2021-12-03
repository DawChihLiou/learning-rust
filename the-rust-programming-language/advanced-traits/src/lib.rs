//
// Specifying placeholder types in trait definitions with associated types
//
// Associated types connect a type placeholder with a trait such that the trait method definitions
// can use these placeholder types in their signatures.
//
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
