use core::time;
use std::{thread, time::Duration};
use rand::{thread_rng, Rng};


#[tokio::main]
async fn main() { // Future , poll
    let f = say_hello().await;// 
    
    // let t = thread::spawn(||{
    //    thread::sleep(time::Duration::from_millis(500));
    //     println!("I am thread!")
    // });
    // println!("Hello World")
}

async fn say_hello(){
    println!("Hello Impact Analytics!");
    let a = get_rand_num().await; // pending 
    let b = get_rand_num().await;
    let r = calc(a, b).await;
    println!("result:{}",r);
}

async fn calc(a:i32,b:i32)->i64{
    (a+b)as i64
}

async fn get_rand_num()->i32{
    let s =tokio::time::sleep(Duration::from_millis(1000));
    s.await;
    let mut rng = thread_rng();
               rng.gen_range(1..=100)     
}