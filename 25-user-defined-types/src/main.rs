pub mod shapes_rect;

pub mod greet;

mod shapes_square;

use shapes_rect::rect::Rect;
use shapes_square::square::Square;

use crate::greet::Greet;



fn main() {
   //let r1 = Rect{L:12.23,B:34.34};

    let mut r1 = Rect::new(123.34, 456.34);

    let r3 = Rect::defaults();
    Greet();

    let a1 = r1.area();


    let p1 = r1.perimeter();

    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

    let s1 = Square::new(23.54);

    let a1 = s1.area();

    let p1 = s1.perimeter();
    
    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

    
}

// Rust is not object oriented