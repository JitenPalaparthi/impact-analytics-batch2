use crate::shapes::Shapes;
use crate::shapes::What;

pub struct Square(f32);

impl Square {
    pub fn new(s: f32) -> Self {
       Self(s)
    }
}

impl Shapes for Square {
    fn area(&self) -> f64 {
        (self.0 * self.0) as f64
    }
    fn perimeter(&self) -> f64 {
        (self.0 * 4.0) as f64
    }
}
impl What for Square{}
