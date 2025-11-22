pub trait Shapes {
     fn area(&self) -> f64;
     fn perimeter(&self) -> f64;
}
// Can do the default implementation

pub trait What { // default implementaion
     fn what(&self) -> String{
          "Shape".to_string()
     }
}
