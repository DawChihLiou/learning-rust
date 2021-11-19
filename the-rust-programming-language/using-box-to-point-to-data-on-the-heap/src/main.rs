//
// `Box<T>` is the most straightfoward smart pointer. It allow
// us to store data in the heap rather than the stack.
//
fn main() {
    // `5` is allocated in the heap. 'b` is assigned the `Box` that
    // points to its value `5` in the heap.
    let b = Box::new(5);
    println!("b = {}", b);

    use crate::List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

//
// Enabling Recursive types with boxes.
//
// We use `Cons` to store a `i32` and a smart pointer `Box<List>` to
// link to the next list item. `Nil` indicates there is no next item.
//
// "Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size
// doesn’t change based on the amount of data it’s pointing to. This means we can put a Box<T>
// inside the Cons variant instead of another List value directly. The Box<T> will point to the
// next List value that will be on the heap rather than inside the Cons variant." - The Rust
// Programming Language
//
enum List {
    Cons(i32, Box<List>),
    Nil,
}
