use std::thread;
use std::time::Duration;
fn main() {
    // t1

    let h1 = thread::spawn(|| {
        for i in 1..=10 {
            println!("Thread-2 -->{}", i);
            thread::sleep(Duration::from_millis(500));
        }
    }); // t2

    let h2 = thread::spawn(|| {
        for i in 1..=10 {
            println!("Thread-3 -->{}", i);
            thread::sleep(Duration::from_millis(500));
        }
    }); // t3

    for i in 1..=10 {
        println!("Thread-1 -->{}", i);
        thread::sleep(Duration::from_millis(100));
    }

    let r1 = h1.join();

    match r1{
        Ok(d)=>println!("{:?}",d),
        Err(e)=>println!("somethig went wrong :{:?}",e)
    }
    h2.join().unwrap();
    // asking main thread to join those two handlers, when they finish execution
}

// FnOnce -->Function Type --> move , the ownership shold be moved into the thread
//

// main is also a thred 
// main would not wait for other threads to complete their execution by default/design