use std::fmt::Display;
use std::{rc::Rc, str::FromStr};
use std::ops::Drop;
fn main() {
    {
        let s = Box::new(MyString("Hello World".to_string())); // what is the life of s
         println!("{:?}",s);
         // Drop is called
    }

    let mut str1: MyString = MyString("Hello Impact Analytics".to_string());
    
    println!("{:?}",str1);

    let rc1: Rc<MyString> = Rc::new(str1);// 1

    let rc2 = Rc::clone(&rc1); //2

    println!("number of count of rc1:{}",Rc::strong_count(&rc1));

    println!("{:?} {:?}", rc1, rc2);
    {
        let rc3 = Rc::clone(&rc1); //3
        let rc4 = Rc::clone(&rc3); //4
        println!("{:?} {:?}", rc3, rc4);
        println!("number of count of rc1:{}",Rc::strong_count(&rc1));

    } // 2
     println!("{:?} {:?}", rc1, rc2);
     println!("number of count of rc1:{}",Rc::strong_count(&rc1)); //2

}


#[derive(Debug)]
struct MyString(String);

impl std::ops::Drop for MyString { // kind of a destructor
    fn drop(&mut self) {
        println!("Dropping String!-->{}",self.0);
    }
}




// How many owners --> One owner for every data
// What if I want to have multiple owners -->
// When to deallocate --> since there is no GC .. how does it that to deallocate memory
