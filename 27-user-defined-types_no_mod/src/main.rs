pub mod shapes;

use shapes::rect::rect::Rect;
use shapes::square::square::Square;
use shapes::circle::circle::Circle;




fn main() {
   //let r1 = Rect{L:12.23,B:34.34};

    let mut r1 = Rect::new(123.34, 456.34);

    let r3 = Rect::defaults();
  
    let a1 = r1.area();


    let p1 = r1.perimeter();

    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

    let s1 = Square::new(23.54);

    let a1 = s1.area();

    let p1 = s1.perimeter();
    
    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

     let c1 = Circle::new(23.54);

    let a1 = c1.area();

    let p1 = c1.perimeter();
    
    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

    
}

// Rust is not object oriented