pub mod rect;

use rect::Rect;
fn main() {
   
  let r1 = Rect::new(12.4, 13.6);
  let r2 = Rect::new(22.4, 23.6);
  let r3 = r1+r2; // The ownership is transferred to + operator 
  r3.print();

 // let r3=r1.add_both(&r2);
 //r3.print();


}
