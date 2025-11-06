use std::arch::naked_asm;

fn main() {
   
   let b: Box<i32> = get_sq1(100); 

   println!("{}",b);

   // variable is dropped here .. instad of drop in the function
}


fn get_sq1(n:i32)->Box<i32>{  // smart pointer, the ptr is allocared on heap, malloc kind of heap allocation
    let r = n *n;
    return Box::new(r)
}// heap would not be deallcarted

// fn get_sq2(n:i32)->&i32{ // am I using any raw pointer here?
//         let ret = n *n ;
//         return &ret; // This is a dangling reference
//     } // deallocated

    // Go code .. what go does ? 
    // What is the problem with the implementation?
    // func get_sq(n int)*int{
    //     r := new(int) // variable is created on stack 
    //     *r = n *n // dereferenced 
    //     return r // return the pointer
    //     // since the variable is created on stack, its lifetime is with the stack frame.
    // } // in Golang the r would be automatically moved to heap memory, since heap lives as long as it is dropped or gc would deallocat