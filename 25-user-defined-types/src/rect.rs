
pub struct Rect{
    pub L:f32,
    pub B:f32,
}

// self, Self
impl Rect{
    // similar to constructures
    // the below are not methods
    pub fn new(l:f32,b:f32)->Rect{
        Self { L: l, B: b }
    }
}

impl Rect{
    pub fn defaults()->Self{
        Rect { L: 1.0, B: 1.0 }
    }
}

impl Rect{
    pub fn area(&self)->f64{
        (&self.L * &self.B) as f64
    }

     pub fn perimeter(&self)->f64{
        (2.0* (&self.L + &self.B)) as f64
    }
}


// any .rs file that is created in rust is considered as a separate module