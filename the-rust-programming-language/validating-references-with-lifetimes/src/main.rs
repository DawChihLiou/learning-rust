/*
 * "Rust requires us to annotate the relationships using
 * generic lifetime parameters to ensure the actual references
 * used at runtime will definitely be valid."
 *
 * A "borrow checker" compare scopes at compile time to check
 * for dangling reference.
 *
 * When a function is borrowing, the lifetime of the references
 * can be explicitly annotated so that the compiler can make
 * sure the references will be valid at runtime.
 *
 * "Ultimately, lifetime syntax is about connecting the lifetimes
 * of various parameters and return values of functions."
 *
 * Structs can hold references. Each reference should add a
 * lifetime annotation. For example:
 *
 * ```
 *  // The struct has a string slice reference. It means the struct
 *  // won't outlive the reference.
 *
 *  struct ImportantExcerpt<'a> {
 *      part: &'a str,
 *  }
 * ```
 *
 * Lifetime elision rules are the common usecases that are programmed
 * into Rust compiler. For lifetimes, the compiler considers the
 * input lifetimes(function parameters) and output lifetimes(return
 * values). If it is deterministic, it will compile.
 *
 * Three lifetime elision rules
 *
 *  1. Each parameter gets its own lifetime parameter.
 *  2. If there's only one input lifetime paramter, the lifetime is
 *     assigned to all output lifetime parameters.
 *  3. If one of the input lifetime parameters is `&self` or `&mut self`,
 *     the lifetime of self is assigned to all output lifetime paramters.
 *
 * `'static` is a special lifetime. It tells that the reference lives for
 * the entire duration of the program.
 *
 *
 */

// Enforce &x, &y, and the reference of the return slice to
// have the same lifetime.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
 * generic type prarameters, trait bounds, and lifetimes togehter
 */
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let another_result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", another_result);
}
