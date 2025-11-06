fn main() {

    let x = 1; // Expression
    
    let y = (x + 100 / 20 * 50 + (x+1)) > 10 && true || false; // Expression

    // Arthimetic
    // Comparision
    // && || --> Logical
    // << >> & | ^ -> Bitwise operators

    println!("{}",y);

    // A scope or a block can also be evaluated as an expression
    let result ={
        let a = 100;

        let b = 200;

       // return a+b; // return can be used only in functions, so this is an error
       a+b
    };
    println!("Result:{}",result);
    let result: () = {
        let (a,b)=(10,20); // Tuple

        println!("{} {}",a,b);
    }; // this is also an expressions , but it returns nothing
     println!("Result:{:?}",result);

     let num = 100; 
     if num%2==0{
        println!("{} is even",num);
     }else{
        println!("{} is odd",num);
     }  

     let mut result = ""; // Type inference
      result = if num%2==0{
         println!("{} is even",num);
        "even"
     }else{
         println!("{} is odd",num);
        "odd"
     };
     println!("Result:{}",result);

     let result = get_nothing();

    //  let mut x = 100;

    //  let y = &mut x;

    //  *y = 500;// Trait which is called Deref

     // Anonymous / Closure 
     // Purely depends on the Trait it implements
     // Fn, FnMut, FnOnce --> Traits 

    

    //  for{

    //  }
    //  while(true){

    //  }
     let mut num = 1;
     loop{ // infinet loop
        if num>10{
            break;
        }
        print!("{} ",num);
        num+=1;
     }

     let mut num=1;
     while(num<=10){
         print!("{} ",num);
         num+=1;
     }

     for i in 1..=10{ // This is used for the collections or iter trait
        print!("{} ",i);
     }


     for i in 1..=10{
        if i%2!=0{
            continue;
        }
        print!("{} ",i);
     }


     let mut done = false;
     for i in 1..=10{
        if done{
            break;
        }
        for j in 1..=10{
           if j==5{
            done=true;
            break;
           } 
           println!("i:{} j:{}",i,j);
        }
     }

     println!("Break outer");
     'outer: for i in 1..=10{
        for j in 1..=10{
            if j==5{
                break 'outer
            }
             println!("i:{} j:{}",i,j);
        }
     }

     // 

     // normal loop

     let mut num = 1;
     let result = loop{
      num *=2;
      if num >50{
         break num;
      }
     };

}

fn get_nothing(){
    println!("There is nothing");
}

// ret := func(){
//     a,b:= 10,20;
//     return a+b;
// }()

// There is no zero value in rust
// Zero value is default value
// Value inference and type inference

// Create a simple if  elseif else block .. give a value and return
// whether the value is even / odd or a prime number