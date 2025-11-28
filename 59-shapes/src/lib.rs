pub mod circle;
pub mod rectangle;


pub use circle::Circle;
pub use rectangle::Rectangle;

/// Common behavior for shapes.
pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

/// A small convenience macro to build shapes.
///
/// ```
/// use shapes::{Shape, shape};
///
/// let c = shape!(circle: 2.0);
/// assert!((c.area() - std::f64::consts::PI * 4.0).abs() < 1e-6);
/// ```
#[macro_export]
macro_rules! shape {
    (circle: $radius:expr) => {
        $crate::Circle { radius: $radius }
    };

    (rect: $width:expr, $height:expr) => {
        $crate::Rectangle { width: $width, height: $height }
    };
}

macro_rules! hello{
    ()=>{
        println!("Hello World");
    };
}

macro_rules! add {
    ($a:expr,$b:expr) => {
        $a + $b
    };
   // $($x:expr),+ $(,)?
}

pub fn Greet(){
    println!("Hello Impact Analytics");
}

// expr
// ide
// ty 



#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape;

    #[test]
    fn circle_area() {
        let c = Circle::new(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-6);
       
    }

    #[test]
    fn macro_creates_circle() {
        let c = shape!(circle: 2.0);
        assert_eq!(c.radius, 2.0);
        let c= add!(10,20);
       // let c = add!(true,false);
    }

    #[test]
    fn macro_creates_rect() {
        let r = shape!(rect: 3.0, 4.0);
        assert_eq!(r.width, 3.0);
        assert_eq!(r.height, 4.0);
    }
}