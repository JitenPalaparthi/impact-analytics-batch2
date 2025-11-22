use crate::shapes::Shapes;
pub struct Rect {
    L: f32,
    B: f32,
}

impl Rect {
    pub fn new(l: f32, b: f32) -> Self {
        return Self { L: l, B: b };
    }
}

impl Shapes for Rect {
     fn area(&self) -> f64 {
        (self.L * self.B) as f64
    }
     fn perimeter(&self) -> f64 {
        (2.0 * (self.L + self.B)) as f64
    }
}
