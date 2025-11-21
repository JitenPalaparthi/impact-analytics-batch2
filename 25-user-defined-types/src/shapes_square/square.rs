pub struct Square(f32);

impl Square{
    pub fn new(s:f32)->Self{
        Self(s)
    }
}

impl Square{
    pub fn area(&self)->f64{
        return (&self.0 * self.0) as f64;
    }

    pub fn perimeter(&self)->f64{
         (&self.0 * 4.0) as f64
    }
}