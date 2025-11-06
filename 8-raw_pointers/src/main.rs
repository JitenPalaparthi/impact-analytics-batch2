static mut GLOBAL:i32=0; 
fn main() {
    let mut num = 123;

    let ptr_num = &num as *const i32; // Immutable Raw Pointer

    let ptr_mut_num = &mut num as *mut i32; // Mutable Raw Pointers

    println!("{:p}",ptr_num);

    unsafe{
        *ptr_mut_num = 1231;
    }
    // deallocated since it is stored in the stack 

    let ptr:*const i32= std::ptr::null(); // a pointer can be null , only if it is a raw pointer
    println!("{:p}",ptr);
    let ptr_mut:*mut i32 = std::ptr::null_mut();
    println!("{:p}",ptr_mut);

    // unsafe{
    //     *ptr_mut = 100; // cannot dereference a null pointer since there is no address is given to the ptr
    //      println!("{}",*ptr_mut);
    // }

    let ret: *const i32 = get_sq1(14); // It returns a raw pointer
    unsafe{
        println!("What has return here --> {}",*ret)
    }

    get_sq3(14);
    println!("{}",unsafe{GLOBAL});

}
 fn get_sq1(n:i32)-> *const i32{
        let ret = n *n; // what is the lifetime of ret ?
        return &ret as *const i32;
       // return std::ptr::null()
    }

   /*  fn get_sq2(n:i32)->&i32{ // am I using any raw pointer here?
        let ret = n *n ;
        return &ret; // This is a dangling reference
    }
    */

    // 'static
      fn get_sq3(n:i32)->*const i32{ // am I using any raw pointer here?
            unsafe{
                GLOBAL = n *n ;
            }
          return &raw const GLOBAL; // This is a dangling reference
          //  return  &mut GLOBAL as *const i32; 
    }


// raw pointers
// a pointer that cannot be dropped by itself 
// a pointer can be null or dangling 
// does not adhere to rust safty checks
// must be used inside unsafe block
// immutable and mutable raw pointers


// raw pointers vs smart pointers
// references --> immutable and mutable reference


// error vs panic