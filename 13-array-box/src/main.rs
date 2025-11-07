fn main() {
  
  // let arr1 = [10,20,30,40,50];

  let arr_box = Box::new([10,20,30,40,50]); // Box is a smart pointer

  let ptr = arr_box.as_ptr();
  // Try to do the ptr arithmetic 

  println!("ptr:{:p} box ptr:{:p} ",ptr,&arr_box);

}

// 
