use std::{thread, time::Duration};
fn main() {
    let mut counter = 1; // It it is a core type and also copy type
    let mut handlers = Vec::new();
    for h in 1..=5 {
        let handler = thread::spawn(move ||{
            for i in 1..=5{
                thread::sleep(Duration::from_millis(100));
                counter+=1; // a new copy of counter is created and that gets incremented
                 println!("handler :{} i:{} counter:{}",h,i,counter);
            }
        });
        handlers.push(handler);
    }

    for h in handlers{
        h.join().unwrap();
    }
    println!("Completed all threads execution:{}",counter);
}
