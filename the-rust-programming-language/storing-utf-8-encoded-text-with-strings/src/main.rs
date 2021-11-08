fn main() {
    //
    // Rust has only one string type in the core: string slice `str`.
    // `String` is implemented as a standard library.
    //

    // creating a new string with `String::new()` 
    let mut s1 = String::new();

    // or creating from a string literal
    let data = "initial content";
    let s2 = data.to_string();
    
    // or 
    let s3 = "initial content".to_string();

    // or with `String::from()``
    let s4 = String::from("initial content");


    // updating a string with `push_string()`
    let mut s5 = String::from("foo");
    // push_str takes a string slice as the paramter so
    // it doesn't take over the ownership of the parameter.
    s5.push_str("bar");

    // or with `push` to add a single character
    let mut s6 = String::from("lo");
    s6.push('l');

    // concatenating with + operator
    let s7 = String::from("Hello, ");
    let s8 = String::from("world!");
    // concatenation with + calls an `add` method under the hood. 
    // The signature looks something like:
    // ```
    //  fn add(self, s: &str) -> String
    // ```
    // What happens to `s7`: 
    //  ownership moves to `add()`. So it will become
    //  invalid after the operation.
    // What happens to `&s8`:
    //  `add()` makes a copy of s8 and coerces the type from 
    //  string to string slice, from `&s2` to `&s2[..]`. 
    //  This is called "deref coercion".
    // What happens to `s9`:
    //  It recieves the owership from the result of the + operation.
    //  It has the type of `String`.
    let s9 = s7 + &s8; // s7 is moved here and become invalid

    // concatenating with `format!` macro
    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");
    // `format!` return a `String`.
    // `format!` is much cleaner than + operator and it doesn't take 
    // ownership of the parameters.
    let s13 = format!("{}-{}-{}", s10, s11, s12);

    //
    // String is a wraper over a Vec<u8>. 
    // Accessing a string through index will result in error.
    //
    // let hello = "Здравствуйте";
    // let answer = &hello[0]; <- [compile error]
    //
    // In UTF-8, there are three ways to look at strings from 
    // Rust's perspective:
    //  * bytes (u8)
    //  * scalar values (Unicode scalar. e.g. `char` type)
    //  * grapheme clusters (clusters of the smallest contrastive 
    //    unit in a writing system)
    //
    
    // slicing strings
    let hello = "Здравствуйте";
    let s14 = &hello[0..4]; // Зд
    println!("{}", s14);

    // iterating over strings
    // operating with chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // operating with bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
