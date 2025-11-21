
pub trait Math{
    fn sq(self)->i64;
}

impl Math for i32{
     fn sq(self)->i64{
        (self * self) as i64
    }
}

pub struct Integer(i32); // user defined type

impl Integer{
    pub fn new(v:i32)->Self{
        Self(v)
    }
}
impl Math for Integer{
     fn sq(self)->i64{
        (self.0 * self.0) as i64
    }
}