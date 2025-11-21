pub mod rect;

pub mod greet;

use rect::Rect;

use crate::greet::Greet;


fn main() {
    let r1 = Rect{L:12.23,B:34.34};

    let r2 = Rect::new(123.34, 456.34);

    let r3 = Rect::defaults();
    Greet();

    let a1 = r1.area();


    let p1 = r1.perimeter();

    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);
    
}

// Rust is not object oriented