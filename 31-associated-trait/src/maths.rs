
pub trait Math{
    type Output; // associated type
    fn sq(self)->Self::Output;
}

impl Math for i32{
    type Output=i64;
     fn sq(self)->Self::Output{
        (self*self) as i64
     }
}

impl Math for i64{
    type Output=i128;
     fn sq(self)->Self::Output{
         (self * self) as Self::Output
     }
}
impl Math for f32{
    type Output= f64;
     fn sq(self)->Self::Output{
          (self * self) as Self::Output
     }
}