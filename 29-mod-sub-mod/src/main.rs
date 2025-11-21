pub mod shapes;

fn main() {
   //let r1 = Rect{L:12.23,B:34.34};

    let mut r1 = shapes::rect::Rect::new(123.34, 456.34);

    let r3 = shapes::rect::Rect::defaults();
  
    let a1 = r1.area();


    let p1 = r1.perimeter();

    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

    let s1 = shapes::square::Square::new(23.54);

    let a1 = s1.area();

    let p1 = s1.perimeter();
    
    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

     let c1 = shapes::circle::Circle::new(23.54);

    let a1 = c1.area();

    let p1 = c1.perimeter();
    
    println!("Area:{:.2} Perimeter:{:.3}",a1,p1);

    shapes::rect::What();
    shapes::circle::What();
    shapes::square::What();
    shapes::What();
    
}

// Rust is not object oriented