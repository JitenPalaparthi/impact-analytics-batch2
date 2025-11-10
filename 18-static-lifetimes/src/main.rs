
fn main() {
    const p:f32 = 3.145;// runtime 
    //let v: f32 = 1.4;
    const PI:f32 = p * 12.5;// compile time.. RO Datasegment

    let s1 = get_sq(100);
    
   
let  name: &mut &'static str= &mut "Impact Analytics";  
println!("{:p}",name.as_ptr()) ;
    *name = "Hello World";
    *name=mutate_str(name,"Hello World") ;
    println!("{:p}",name.as_ptr()) ;
}

fn mutate_str(s : &mut &'static str,st:&str)->&'static str{
  //  s = st;
    *s = "Hello Impact Analytics Inc.";
   // *s = st;
    println!("{:p}",s.as_ptr()) ;
    return s;
}

fn say_hi()->&'static str{
    "Hey Everyone!"// It has to be allocated in Datasegment and return here
}

fn say_hi2()->String{
    let st = "Hey Everyone!".to_string();
    return st;
    //It has to be allocated in Datasegment and return here
}

fn get_sq(n:i32)->i32{
    let r = n *n;
    return r;
}