fn main() {
    let x = add_one(5);
    println!("x has the value of {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

