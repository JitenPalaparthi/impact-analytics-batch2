
pub mod maths;
use maths::Math;

fn main() {
    let i1: i32 = 123;
    let s= i1.sq();

    println!("square of i1:{}",s);

    let i1 = maths::Integer::new(123);
    let s= i1.sq();
    println!("square of i1:{}",s);
}

// derived from --> inheritance 
// by using or implementing traits

