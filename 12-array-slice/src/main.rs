fn main() {
  let mut arr1 = [10,12,13,14,15]; // Type inference , 
  let arr2: [i32; 5]=[32,23,5,4,5];
  let mut arr3=[0;5]; 
  let char_arr1 = ['X';10];

  let mut sum = 0;
  for i in arr1{
    sum+=i;
  }
  println!("{}",sum);

  let mut arr2 = arr1; // Is it a transfer/move or copy
  println!("{:?}",arr1);
  println!("{:?}",arr2);

  arr2[4]=400; // Mutability based on index
 
 let sum = sum_array(arr1);
   println!("{}",sum);

 let sum= sum_array(arr2);
  println!("{}",sum);


  // let arr3 = [1,2,3];
  // let sum= sum_array(arr3);

  let slice1 = arr1;// mutable borrow 

  let slice2 = &arr1[0..=2]; // Slice 
  let slice3 = &arr1[..]; // Slice
  
  let mut sum = 0;
  // slice2 into the scope, slice2 is borrowd here 
  for s in slice2{
    sum+= s;
  }
  println!("Sum:{}",sum);

  println!("Sum of arr1:{}",get_sum_of(&arr1[..]));
  println!("Sum of arr1:{}",get_sum_of(&slice1));
    let arr3 = [1,2,3];
  // let sum= sum_array(arr3);
  println!("Sum of arr3:{}",get_sum_of(&arr3));

  let arr2d = [
    [10,20,30],
    [40,50,60],
  ];

  for arr in arr2d{
    for e in arr{
        print!("{} ",e);
    }
  }

  let arr3d: [[[i32; 2]; 2]; 2] = [
    [[1,2],[3,4],],[[5,6],[7,8]],
  ];

}

// String
    // Vec 
    // Map
    // Box and other smartpointers are in Heap memory

// Rust --> Bases on the constructs
// Go --> Based on compiler decision
// Zig --> Not based on the type but majorly based on allocator

// glic/musl --> System allocator --> Malloc 

fn sum_array(arr:[i32;5])->i64{
    let mut sum = 0;
    for i in arr{
        sum+=i;
    }
    return sum as i64;
}

fn get_sum_of(slice :&[i32])->i64{
    let mut sum = 0;
  // slice2 into the scope, slice2 is borrowd here 
    for s in slice{
        sum+= *s;
    }
  sum as i64
}

// &str --> refernce of string or slice of string