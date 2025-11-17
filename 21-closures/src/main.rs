use std::prelude::rust_2018;



fn main() {
  
  //let factor = 10;

  // impl vs dyn 

  let closure = |x: i32| x+1 ; // What is the type, 

  let r = closure(10); // This is a variable that is created in this functional stack frame
  
  println!("Result:{}",r);

  let mut factor = 10;

  //let multiply = |x:i32 |*(&factor) *x; // factor is borrowd 
  let mut multiply = |x:i32|->i32{
    factor = factor *x;
    factor
   }; // factor is borrowd 

  // Is using the global variable. 

  let r = multiply(10);
    println!("Result:{}, factor:{}",r,factor);

let mut s1 = "Hello ".to_string(); // Heap allocation

let mut s2: String = "Impact Analytics!".to_string();

let s3 = s1.clone();

//let s4 = s3;
// The ownership is transffered
let mut closure = move |s|{
    s1.push_str(s);
};

println!("s2:{} s3:{}",s2,s3);


let a = 10; // copy . bcz copy trait is implemented.
let c = move |x:i32|a+x;
println!("{}",a);
// let mut v1 = 10;
// let v2 = 30;
// // The ownership of the variable is moved to the function
// let mut closure = move |x: i32|->i32{
//    v1 = x * v1;
//    return x;
// };

// let r = closure(10);

// println!("Result:{} v1:{}",r,v1);

}


// Fn
// FnMut
// FnOnce


// Take a vector give the sum of elements in the vector..

// Fn give sum
// FnMut // add a number to the vector  and five sum
// FnOnce // add a number to the vector and give sum