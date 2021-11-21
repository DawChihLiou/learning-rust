use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // `mpsc` stands for "multiple processors, single consumer", meaning we can have multiple
    // streams of tranmitter and only one reciever in a channel.
    //
    // use `channel` to create communication between threads. We can send data and receive data in the channel.
    let (tx, rx) = mpsc::channel();

    // creating multiple producer by cloning the transmitter.
    let tx1 = tx.clone();

    // spawn multiple threads
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
