fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("heyllo");

    change(&mut s2);
    println!("s1 is changed to {}", s2);
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
}
// Here, s goes out of scope. But because it does not 
// have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world!")
}
