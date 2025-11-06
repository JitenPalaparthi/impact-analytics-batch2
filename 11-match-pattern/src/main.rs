fn main() {
    let num = 100;

    match num {
        0..=20 => println!("{} is between 0-20", num),
        21..=50 => println!("{} is between 21-50", num),
        51..=100 => println!("{} is between 51-100", num),
        _ => println!("{} is above 100", num),
    }

    let day: u8 = 1;

    match day {
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursdat"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("No day"),
    }

    let direction = Direction::East;

    match direction{
        Direction::East => println!("East direction"),
        Direction::West => println!("East direction"),
        Direction::South => println!("East direction"),
        Direction::North => println!("East direction"),
    }

   
    let str1 = ""; // -?
    println!("{:p}",str1.as_ptr());
    let str2 ="";
    println!("length:{} ptr:{:p}",str2.len(),str2.as_ptr());

   //let num =0; // 0 is also a valid value unblike ""
    //let num :&i32= std::ptr::null(); // only if it is a raw pointer
    // 0x00
    // let str1:String ="".to_string();
    // let str1: Option<String>= Option::None;

    let mut num: Option<i32> = Option::Some(100);
    num = Option::None;
    num = Option::Some(100);
   
   match num {
    Some(k)=> println!("{}",k),
    None=>println!("no value or None")
   }
    

   let mut e1:Shape= Shape::None;

   match e1{
    Shape::None => println!("none"),
    Shape::Circle(s)=>println!("Radius of circle:{}",s),
    Shape::Rect { l, b }=>println!("L:{} B:{}",l,b)
   }
   //let e2:Shape= Shape::Circle(100.0);
   //let e3:Shape= Shape::Rect { l: 10.12, b: 12.12 } ;

}


enum Direction{
    East,
    West,
    South,
    North,
}
// Value
// Touple
// Struct

enum Shape{
    Circle(f32), // 4 bytes
    None, // Zero bytes
    Rect{l:f32,b:f32} // 8 bytes
} 
// What is the size the variable of this enum


// Match pattern

// struct T{
// ok:bool, // 1 --> 8 -->7 padding
// num1:i64,// 8 --> 8
// ok2:bool,// 1 --> 8 --> 7 padding
// float1:f64// 8 --> 8
// }

// struct T{
// num1:i64,// 8 --> 8
// float1:f64// 8 --> 8
// ok2:bool,// 1 --> 8 -- 7
// ok:bool, // 1 --> 6
// num2:i32 // 4 bytes
// num3:i16 // 2 bytes
// }

// 2 to the power of 
// 8,16, 32, 128, 16, 
// >> << --> multiplkt by2 or divide by 2
// 5-6 operasions 
// 2 * 2 --> 4-10 instructions 
// 2 >> 2 --> 1 opeartions 