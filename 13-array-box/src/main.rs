fn main() {
    // let arr1 = [10,20,30,40,50];

    let arr_box = Box::new([10, 20, 30, 40, 50]); // Box is a smart pointer

    let ptr = arr_box.as_ptr();
    // Try to do the ptr arithmetic

    println!("ptr:{:p} box ptr:{:p} ", ptr, &arr_box);

    let num_box = Box::new(100); // stack allocation
    println!("{:p}", &num_box);
    let r = get_Sq(num_box);
}

fn get_Sq(num: Box<i32>) -> i64 { // The address on stack is different on this stack frame
    println!("{:p}", &num);
    0
}
