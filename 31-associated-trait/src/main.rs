mod maths;
use maths::Math;
fn main() {
    let i1 = 1000;
    let r1 = i1.sq();
    println!("r1 on i32:{}", r1);

    let i1 = 1000 as i64;
    let r1 = i1.sq();
    println!("r1 on i32:{}", r1);

    let i1 = 12.234 as f32;
    let r1 = i1.sq();
    println!("r1 on i32:{:.3}", r1);
}
