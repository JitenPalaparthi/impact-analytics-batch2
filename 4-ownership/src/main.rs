fn main() {
    let i = 100;
    let j = i; //copy
    println!("{}", i);

    let str1 = "Hello".to_string();
    let str2 = str1; //transfer of the owenerwhip

    // drop(str1);
    println!("{}", str2);

    // 
    let nums= (100,20); // tuple
    //let (a,b )=(100,200);
    let ret=add(nums.0,nums.1);
    println!("{}",ret);

    let l=get_len(str2.clone());
     println!("{}",l);
     println!("{}",str2);

     let i = 100;
     let j = i;

     let str3 = str2.clone();
     
}

//copy

// every value/data has only one owner
// when you change onwership , of a value , the previous owen would become obsolete
//
fn add(a:i32,b:i32)->isize{
    return (a+b) as isize;
}

fn get_len(s:String)->usize{
    s.len()
}