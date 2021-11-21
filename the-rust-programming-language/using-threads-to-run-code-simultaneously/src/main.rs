use std::thread;
use std::time::Duration;

//
// "Concurrent programming" is where different parts of a program execute independently.
// "Parallel programming" is where different parts of a program execute at the same time.
//
fn main() {
    // `thread::spawn` returns a `JoinHandle`. We can call `join` method on it to wait for its
    // thread to finish.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // `join()` wil block the other threads until the handle is completed.
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // use `move` to take ownership of `v` to the spawned thread in `handle_v`.
    // it's to make sure `v` lives throughout the spawned thread.
    let handle_v = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle_v.join().unwrap()
}
