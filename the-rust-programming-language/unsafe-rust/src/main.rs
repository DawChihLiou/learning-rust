//
// Unsafe Superpowers
//
// - Dereference a raw pointer
// - call an unsafe function or mthod
// - access or modify a mutable static variable
// - implement an unsafe trait
// - access fields of `union`s
//
fn main() {
    //
    // dereferencing a raw pointer
    //
    let mut num = 5;
    // use as to cast a immutable and mutable reference into their corresponding raw pointer
    // types.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

//
// calling an unsafe function or method
//
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

//
// using `extern` functions to call external code
//
extern "C" {
    fn abs(input: i32) -> i32;
}

//
// calling Rust funcitons from other languages
//
// `#[no_mangle]` tells the Rust compiler not to mangle the name for this function.
// Mangle: when a compiler changes the function name to another name that contains more
// information during compiling.
//
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

//
// accessing or modifying a mutable static variable
//
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
