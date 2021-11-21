use std::sync::{Arc, Mutex};
use std::thread;

//
// Sharing memory is another way of handling concurrency. However, it's more error prone and tricky
// to handle.
//
fn main() {
    // `Mutex<T>` stands for mutual exclusion. It allows only one thread to access a shared data at a
    // time. When calling `lock`, it returns a smart pointer `MutexGuard`.
    //
    // `MutexGuard` implements `Deref` and `Drop` trait to point to inner data and to release the lock when the `MutexGuard` goes out of scope.
    //
    // `Arc<T>` is like `Rc<T>` that is safe in concurrent situations. Thread safty comes with
    // performance panalty so only use `Arc` when we need it. `Arc<T>` handles multiple ownership.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // aquire lock from `Mutex` to access the data
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
