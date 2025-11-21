pub struct Rect {
    pub L: f32,
    pub B: f32,
     a: f64,
     p: f64,
}

// self, Self
impl Rect {
    // similar to constructures
    // the below are not methods
    pub fn new(l: f32, b: f32) -> Rect {
        Self {
            L: l,
            B: b,
            a: 0.0,
            p: 0.0,
        }
    }
}

impl Rect {
    pub fn defaults() -> Self {
        Rect {
            L: 1.0,
            B: 1.0,
            a: 0.0,
            p: 0.0,
        }
    }
}

impl Rect {
    pub fn area(&mut self) -> f64 {
        self.a = (&self.L * &self.B) as f64;
        return self.a;
    }

    pub fn perimeter(&mut self) -> f64 {
        self.p = (2.0 * (&self.L + &self.B)) as f64;
        self.p
    }
}

// any .rs file that is created in rust is considered as a separate module
