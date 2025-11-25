use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
fn main() {
    let mut counter = Arc::new(Mutex::new(0));
    
    // Why Rc or Arc ? Arc is thread safe
    // Why not Rc?  Rc is not thread safe
    // Why not RefCell
    // What is similarity of Rc and Arc --> Both of them are Reference counters
    // Why Mutex but not RefCell -> RefCell Not implemented Sync byt Mutex has


    // It it is a core type and also copy type
    let mut handlers = Vec::new();
    for h in 1..=5 {
        let c = Arc::clone(&counter); // increment the counter
        let handler = thread::spawn(move || {
            for i in 1..=5 {
                thread::sleep(Duration::from_millis(100));
                let mut num = c.lock().unwrap();// This creates contention
                *num += 1;
                println!("handler :{} i:{} counter:{}", h, i, *num);
            }
        });
        handlers.push(handler);
    }

    for h in handlers {
        h.join().unwrap();
    }
    println!("Completed all threads execution");
}
// Please impolement RWMutex
// make 5 threads to read the valye and two thread to write the value

// instead of mutex try to use atomic variable