fn main() {
    println!("Hello, world!");
    let mut num1:i32 = 1231; 
    let mut num2:u32=0;
    println!("{} {}",num1,num2);
    {
        let num1 = 2322;
        let num2 = 123;
        println!("{} {}",num1,num2);
    }
    let num1= 9999;
    let num2: i128= 33432432;

     println!("{} {}",num1,num2);

     let float1 = 123.123; // type inference
     let float2: f32 = 123.123;

     let ok1 = true;
     let char1 = 'A';
     let str1 = "Hello World";
}

// let 
// let mut 
// const 
// static 