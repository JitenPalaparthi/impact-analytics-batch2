use std::ops::{Index, IndexMut};

fn main() {
    let mut vec1: Vec<i32> = vec![10, 20, 30, 40, 50];

    println!(
        "vec: {:#?} vec raw_ptr:{:p} ptr:{:p} len:{} cap:{}",
        vec1,
        vec1.as_ptr(),
        &vec1,
        vec1.len(),
        vec1.capacity()
    );

    vec1.push(60);
    println!(
        "vec: {:#?} vec raw_ptr:{:p} ptr:{:p} len:{} cap:{}",
        vec1,
        vec1.as_ptr(),
        &vec1,
        vec1.len(),
        vec1.capacity()
    );
    vec1.push(70);
    println!(
        "vec: {:#?} vec raw_ptr:{:p} ptr:{:p} len:{} cap:{}",
        vec1,
        vec1.as_ptr(),
        &vec1,
        vec1.len(),
        vec1.capacity()
    );

    // let mut vec1: Vec<i32> = Vec::with_capacity(100);
    // println!("vec: {:#?} len:{} cap:{}",vec1,vec1.len(),vec1.capacity());

    let mut sum = 0;

    for v in &vec1 {
        // When do you really borrow--? If heap allocated 100% you borrow..
        sum += v;
    }

    println!("sum:{}", sum);
    println!(
        "vec: {:#?} vec raw_ptr:{:p} ptr:{:p} len:{} cap:{}",
        vec1,
        vec1.as_ptr(),
        &vec1,
        vec1.len(),
        vec1.capacity()
    );

    // let arr1 = [0,20,30];
    // let arr2 = arr1; // deep copy | COPY
    // let arr3: &[i32; 3] = &arr1; // shallow copy
    // let arr4 = &arr3.clone();
    // let arr5 = &arr4.clone();

    // let vec1 = vec![10,20,30];
    // let vec2 = vec1.clone(); // deep copy
    // let vec3 = vec2; // Shallow copy

    for v in &mut vec1 {
        *v = *v * 2;
    }

   
   let mut n =0;

   

   let mut v = &mut vec1[0] ;
    *v = 1000;


//    while(n<vec1.len()){
//      let v: &mut i32 =Vec::index_mut(&mut vec1, n);
//      *v = *v *2;
//      n+=1;
//    }


   
    // DeRef --> trait 
    println!(
        "vec: {:#?} vec raw_ptr:{:p} ptr:{:p} len:{} cap:{}",
        vec1,
        vec1.as_ptr(),
        &vec1,
        vec1.len(),
        vec1.capacity()
    );
}

// i , j both of them stack allocations
