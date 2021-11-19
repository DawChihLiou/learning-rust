//
// Dereference operator `*` follows the reference to the value it's pointing to.
//
// ```
// let x = 5;
// let y = &x;
// assert_eq!(5, x);
// assert_eq!(5, *y);
// ```
//
// Smart pointers can behave like reference by implementing `Deref` trait.
//
// Deref coercion calls a series of `deref` methods to coerce the value.
//
// How Deref coercion interacts with mutability? we can use `DerefMut` trait to override the `*`
// operator on mutable references. Rust does deref coercion when:
//
// - From `&T` to `&U` when `T: Deref<Target=U>`
// - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
// - From `&mut T` to `&U` when `T: Deref<Target=U>`
//
fn main() {
    let x = 5;
    // using Box<T> like a reference
    let y = Box::new(x);
    let z = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // `*z` runs `*(z.deref())` when compiling to find the value that MyBox is pointing to.
    assert_eq!(5, *z);

    let m = MyBox::new(String::from("Rust"));

    // Implicit deref coercions with functions and methods
    //
    // `&m is a reference to `MyBox<String>` value. Because `Deref` in implemented on `MyBox<T>`,
    // Rust turn &MyBox<String> into &String by calling `deref`. The Deref implementation by the
    // standard libary on `String` will turn `&String` into `&str`. This is why we can call the
    // `hello()` with `&m`. The implicit coercion is convinient, otherwise we have to call `hello`
    // function like this:
    //
    // ```
    //  hello(&(*m))[..]);
    //  // *m finds the String value MyBox<T> points to
    //  // [..] slices the String
    //  // & indicates the reference of the slice
    // ```
    hello(&m);
}

use std::ops::Deref;

// defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementing `Deref` trait to tell the compiler how to find the value that the pointer is
// pointing to.
impl<T> Deref for MyBox<T> {
    // defines an associated type for the Deref trait to use.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // returns a reference to the value we want to access with the `*`
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
