pub struct Circle(f32);


impl Circle{
   pub fn new(r:f32)->Self{
        Self(r)
    }
}

impl Circle{
    pub fn area(&self)->f64{
        (3.14 * self.0 * self.0) as f64
    }

     pub fn perimeter(&self)->f64{
        (2.0 * 3.14 * self.0) as f64
    }
}