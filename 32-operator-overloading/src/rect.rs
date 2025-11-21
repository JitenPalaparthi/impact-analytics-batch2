use std::ops::{Add,Sub};

pub struct Rect{
    pub L:f32,
    pub B:f32,
}

impl Rect{
    pub fn new(l:f32,b:f32)->Self{
       return Self { L: l, B: b } 
    }
}

impl Add for Rect{
    type Output = Rect;
    fn add(self, other: Rect) -> Rect {
        Self {
            L: self.L + other.L,
            B: self.B + other.B,
        }
    }
}

impl Sub for Rect{
    type Output = Rect;
    fn sub(self, other: Self) -> Self {
        Self {
            L: self.L - other.L,
            B: self.L - other.B,
        }
    }
}

impl Rect{
    pub fn print(&self){
        println!("L:{:.2} B:{:.2}",self.L,self.B)
    }

    pub fn add_both(&self, other: &Rect) -> Rect {
        Self {
            L: self.L + other.L,
            B: self.B + other.B,
        }
    }
}
