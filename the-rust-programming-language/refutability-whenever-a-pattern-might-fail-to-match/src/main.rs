//
// There are "refutable" patterns and "irrefutable" patterns.
// Patterns that will match for any possible value are irrefutable.
//
fn main() {
    let some_option_value: Option<i32> = None;
    //  `let` requires irrefutable pattern. `Option<i32>` can be `None` so we will
    // get a compile error.
    // let Some(x) = some_option_value;

    // so we should use `if let` to handle refutable patterns
    if let Some(x) = some_option_value {
        // `some_option_value` is `None` so the code will skip the body here.
        println!("{}", x);
    }

    // we will get a warning for useless `if let` because the pattern is irrefutable.
    if let x = 5 {
        println!("{}", x);
    }
}
