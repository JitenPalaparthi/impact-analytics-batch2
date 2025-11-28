use std::pin::Pin;

fn main() {
    let mut x = Box::new(10);
    let pinned = Pin::new(&mut x);

    // You can modify the value
    *pinned.get_mut() = Box::new(20);

    // But you cannot move x out of pinned safely
}