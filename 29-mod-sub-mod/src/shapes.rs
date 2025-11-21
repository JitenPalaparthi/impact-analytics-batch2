pub mod rect{
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

pub fn What(){
    println!("It is rect module");
}
}

pub mod square{
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
pub fn What(){
    println!("It is square module");
}
}

pub mod circle{
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

pub fn What(){
    println!("It is circle module");
}
}

pub fn What(){
    println!("It is shapes module");
}
