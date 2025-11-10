use std::fmt::{Display,Formatter,Result};
fn main() {
   
   let mut c = Circle(123.12);
   println!("Circle c:{}",c.0);
   c.0 = 423.34;

   println!("Circle:{}",c);

   let e1:Empty=Empty{};
   let e2:Empty=Empty{};

   let e3 = e1; // 


   println!("Empty:{} Address;{:p}",e1,&e1);
   println!("Empty:{} Address;{:p}",e2,&e2);
} 

// "Circle Radius=12.32"

fn get_area(c:&Circle)->f32{
    return 2.0*3.14*c.0;
}

//#[derive(Debug)] // annotation
struct Circle(f32); // Touple structure


impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(Circle Radius {})", self.0)
    }
}

#[derive(Clone, Copy)]
struct Empty; // Unit structure

impl Display for Empty{
fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "it is Empty")
    }
}


