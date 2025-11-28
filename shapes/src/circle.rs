use crate::Shape;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}


// pub mod shape_square{
//    pub struct square(f32);
//    impl square {
//     pub fn new(radius: f32) -> Self {
//        square(radius)
//     }
// }
// }