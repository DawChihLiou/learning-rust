/*
 * All interators implement the `Iterator` trait. It looks like:
 *
 * ```
 *  pub trait Iterator {
 *      type Item;
 *
 *      fn next(&mut self) -> Option<Self::Item>;
 *
 *      // ...
 *  }
 * ```
 *
 * Iterator implementations requires a `next` method.
 * `next()` returns `Some(value)` when value is available.
 * It returns `None` when reaching the end of the collection.
 *
 * See `src/lib.rs` for example.
 */
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // consuming iterator
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // consuming custom iterator
    let mut counter = Counter::new();

    println!("counter: {:?}", counter.next());
    println!("counter: {:?}", counter.next());
    println!("counter: {:?}", counter.next());
    println!("counter: {:?}", counter.next());
    println!("counter: {:?}", counter.next());
    println!("counter: {:?}", counter.next());

    // using other iterator trait methods
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // (0, 1), (1, 2), (2, 3), (3, 4), (4, 5)
        .map(|(a, b)| a * b) // 0, 2, 6, 12, 20
        .filter(|x| x % 3 == 0) // 6, 12
        .sum(); // 18
                /*
                 * Note:
                 * `zip` returns `None` when either of its input iterators
                 * return `None`. So the tuple (5, None) was not produced.
                 */

    println!("Sum: {}", sum)
}

#[derive(PartialEq, Debug)]
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
