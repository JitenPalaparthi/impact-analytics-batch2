use std::ops::Add;

fn main() {
   let _vals=(100,true,"hello World");
   let (_a,_,_b)=(100,true,"hello World");
    // _ blank identifier, return or assign no value
   _ = 100;
   let _x= 100;

   let r1={
        println!("{}" ,100 *100)
    };
    let r2={
        println!("{}" ,100 *100)
    };

     _ ={
        println!("{}" ,100 *100)
    };

    let _t = ();// it has got a memory

    println!("r1:{:p} r2:{:p} _t:{:p}",&r1,&r2,&_t);

   println!("{}",std::mem::size_of_val(&_x));

   let b1 = Box::new(100);
   let b2:Box<String>= Box::new("Hello World".to_string());

    println!("{}",std::mem::size_of_val(&b1));
    println!("{}",std::mem::size_of_val(&b2));
    
    get_type_of(&b1);
    get_type_of(&b2);
    get_type_of(&r1);

    //et i1:int32= 100;

    //get_type_of(&i1);

    add(10i8,20i8);
    add(1212312.12123123,1012312.23123123123);
    add(12.0f32,12.5f32);
    add(112312321i64,12312331i64);
    //add(true,false);

}

type int32 = i32; // type alias
type double = f64;
type float = f32;
type bigint=i128;


fn get_type_of<T>(_:&T){ // Blank identifer bcz there is no need of the actual variable , just need its type
    println!("Type:{}", std::any::type_name::<T>())
}
// std::any::type_name



fn add<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

fn calc(a:i32,b:i32)->(i64,i64,i64){
    return ((a+b) as i64,(a-b) as i64,(a*b) as i64)
}

