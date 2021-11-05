fn main() {
    // gives_owership moves its return value into s1
    let s1 = gives_ownership();
    // s2 comes into scope
    let s2 = String::from("hello");
    // s2 is moved into takes_and_gives_back, which also moves
    // it's return value into the function that calls it
    let s3 = takes_and_gives_back(s2);
    
    println!("power is {}, {}", s1, s3)
}
// Here, s3 goes out of scope and is dropped. s2 was moved, so 
// nothing happens. s1 goes out of scope and is dropped


// gives_ownership will move its return value into the function 
// that calls it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("yours");
    // some_string is returned and moves out to the calling 
    // function 
    some_string
}


// This function takes a String and returns one
// a_string comes into scope
fn takes_and_gives_back(a_string: String) -> String {
    // a_string is returned and moves out to the calling function
    a_string
}
