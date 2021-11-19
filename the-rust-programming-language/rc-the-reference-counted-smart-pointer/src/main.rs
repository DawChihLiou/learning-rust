enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

//
// Use `Rc<T>` for multiple ownership.
//
// "For example, in graph data structures, multiple edges might point to
// the same node, and that node is conceptually owned by all of the edges
// that point to it. A node shouldn’t be cleaned up unless it doesn’t
// have any edges pointing to it." - The Rust Programming Language.
//
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // `Rc::clone` doesn't make deep copy, it increments the reference count.
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
