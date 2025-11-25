use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(5); // there is no limit for the bound

    let produce_handler = thread::spawn(move || {
        for i in 1..=20 {
            tx.send(i).unwrap();
        }
        println!("Hello i am the sender");
    });

    let consumer_handler = thread::spawn(move ||{
        for r in rx{
            thread::sleep(Duration::from_millis(500)); // block
            println!("Received value:{}",r);
        }
    });

    produce_handler.join().unwrap();
    consumer_handler.join().unwrap();
}
