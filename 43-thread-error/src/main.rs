use std::{any::type_name, process::id, thread, time::Duration};
use rand::prelude::*;
fn main() {
    
    let mut rng = rand::rng();
    let  num = rng.random_range(1..=600);

    let p1 = Person{id:100,name:"Jiten".to_string()};

    let mut id = 1001;
    let r1 = Rawid{ptr:&mut id as *mut i32};

    let h1 = thread::spawn(move ||->Result<i32,String>{
        thread::sleep(Duration::from_millis(2000));
        if num%2!=0{
            Err("The rand number is an odd number".to_string())
        }else{
            Ok(num)
        }
    });

    let h2 = thread::spawn(move ||{
        thread::sleep(Duration::from_secs(2));
        println!("{:?}",p1);
    });


      let h3 = thread::spawn(move ||{
        thread::sleep(Duration::from_secs(2));
        println!("Rawid:{:?}",r1);
    });


        match h1.join(){
            Ok(result)=>match result{
                Ok(v)=>println!("Result :{}",v),
                Err(s)=>println!("{}",s)
            }
            Err(_)=>println!("some thing went wrong")
        }
        h2.join().unwrap();

        h3.join().unwrap();
    
    println!("Thread has completed execution");
    
}



#[derive(Debug)]
struct Person{
    id:i32, // This is already implemented send
    name:String, // This also has implemented send
}

#[derive(Debug)]
struct Rawid{
    ptr: *mut i32, // unsafe pointer
}
unsafe impl Send for Rawid{}

// How a hread can give an error 
// Send and Sync
// Arc , Mutex 
// Channels 
