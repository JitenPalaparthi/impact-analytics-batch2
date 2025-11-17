use std::any::Any; // any time
use std::collections::HashMap;

fn main() {
    let f1 = Funcs::Fn1(greet);
    let f2 = Funcs::Fn2(sq);
    let f3 = Funcs::Fn3(add);
    let func_arr: [Funcs; 3] = [f1, f2, f3];
    let (a, b) = (100, 200);

    for f in func_arr{
        match f {
            Funcs::Fn1(fn1)=>fn1(),
            Funcs::Fn2(fn2)=>{
                let r = fn2(10);
                println!("result:{}",r)
            },
            Funcs::Fn3(fn3)=>{
                let r =fn3(a,b);
                 println!("result:{}",r);
            }
        }
    }

    // another way using Any
    let mut func_map:HashMap<&str,Box<dyn Any>>=HashMap::new();

    func_map.insert("greet", Box::new(greet as fn()));
    func_map.insert("sq", Box::new(sq as fn(i32)->i64));
    func_map.insert("add", Box::new(add as fn(i32,i32)->i64));


    

    if let Some(any_fn)=func_map.get("greet"){
       if let Some(fn1)= any_fn.downcast_ref::<fn()>(){
        fn1();
       }
    }

     if let Some(any_fn)=func_map.get("sq"){
       if let Some(fn1)= any_fn.downcast_ref::<fn(i32)->i64>(){
        let r=fn1(100);
        println!("sq :={}",r);
       }
    }

     if let Some(any_fn)=func_map.get("add"){
       if let Some(fn1)= any_fn.downcast_ref::<fn(i32,i32)->i64>(){
        let r=fn1(a,b);
        println!("add:={}",r);
       }
    }


}

enum Funcs {
    Fn1(fn()),
    Fn2(fn(i32) -> i64),
    Fn3(fn(i32, i32) -> i64),
}

fn greet() {
    println!("Hello World!")
}
fn sq(i: i32) -> i64 {
    return (i * i) as i64;
}
fn add(a: i32, b: i32) -> i64 {
    return (a + b) as i64;
}
