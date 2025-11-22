use std::fmt::Display;
use std::{rc::Rc, str::FromStr};
use std::ops::Drop;
use std::cell::RefCell;
fn main() {
    {
        let s = Box::new(MyString("Hello World".to_string())); // what is the life of s
         println!("{:?}",s);
         // Drop is called
    }

    let mut str1: MyString = MyString("Hello Impact Analytics".to_string());

   // let mut str2 = str1.clone();
    
   // println!("{:?}",str1);

    let rc1 = Rc::new(RefCell::new(str1));// 1
    let rc2 = Rc::clone(&rc1);// count increse
    let rc3 = Rc::clone(&rc1);

    *rc1.borrow_mut()=MyString("Hello World".to_string());
    println!("{:?}",rc1);

    *rc2.borrow_mut()=MyString("Hello Earth".to_string());
    println!("{:?}",rc1);

    *rc3.borrow_mut()=MyString("Hello Universe".to_string());
    println!("{:?}",rc1);
}


#[derive(Debug,Clone)]
struct MyString(String);

impl std::ops::Drop for MyString { // kind of a destructor
    fn drop(&mut self) {
        println!("Dropping String!-->{}",self.0);
    }
}



// RC is not thread safe
// If threads safe reference counting --> arc

// How many owners --> One owner for every data
// What if I want to have multiple owners -->
// When to deallocate --> since there is no GC .. how does it that to deallocate memory
