fn main() {
    // let arr1 = [10,20,30,40,50];

    let arr_box = Box::new([10, 20, 30, 40, 50]); // Box is a smart pointer

    let ptr = arr_box.as_ptr();
    // Try to do the ptr arithmetic

    println!("ptr:{:p} box ptr:{:p} ", ptr, &arr_box);

    let mut num_box = Box::new(10); // stack allocation
    println!("ptr of box {:p} raw_ptr:{:p}", &num_box,num_box);
   
    num_box = get_Sq3(num_box);
    println!("ptr of box {:p} raw_ptr:{:p}", &num_box,num_box);
    // Box has not implemented copy // for the raw pointer
    get_Sq1(&mut num_box);
    println!("{}",*num_box);

    get_Sq2(num_box); // The ownership is moved --> Traits
   // println!("{}",*num_box);
    // Box has implemented Copy


    // let s1: String = "hello Wrold".to_string();
    // let s2: &String = &s1; // Ownership is move or transfer fo the data
    // println!("{}",s1)

    // They consider the box is purely heap allocation

}

fn get_Sq1(mut num: &mut Box<i32>) { // The address on stack is different on this stack frame
    println!("ptr of box {:p} raw_ptr:{:p}", &num,num);
    **num = **num * **num;
}

fn get_Sq2(mut num: Box<i32>) { // The address on stack is different on this stack frame
    println!("ptr of box {:p} raw_ptr:{:p}", &num,num);
    *num = *num * *num;
}

fn get_Sq3(mut num: Box<i32>)->Box<i32> { // The address on stack is different on this stack frame
    println!("ptr of box {:p} raw_ptr:{:p}", &num,num);
    *num = *num * *num;
    return num
}

