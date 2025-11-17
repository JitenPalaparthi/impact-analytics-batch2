use std::collections::HashMap;
fn main() {
    let b1 = based_on_flag(true);

    let r = b1(100);
    println!("r:{}", r);

    let b1 = based_on_flag(false);

    let r = b1(100);
    println!("r:{}", r);

    let func = |x: i32| -> i32 {
        return x * x;
    };

    let func_ref = &func;
    let r = *(&func_ref(100));
    let r = func_ref(100);

    //(||{println!("Hello World")})()

    let r = (|a: i32, b: i32| -> i32 {
        return a + b;
    })(10, 20); // executes becase there is an executor
    println!("r:{}", r);

    let f = |a: i32, b: i32| -> i32 {
        return a + b;
    }; // Retuns the function pointer to f

    println!("r:{:?}", f(100, 200));

    let mut func_map: HashMap<&str, fn(i32, i32) -> i32> = HashMap::new();

    func_map.insert("add", add);
    func_map.insert("substract", substract);
    func_map.insert("multiply", |a, b| -> i32 { a * b });

    // let v = &100;

    let (a, b) = (100, 200);

    for (k, v) in func_map {
        println!("Key:{} Value:{} ", k, v(a, b));
        let mut func_vec = vec![add, substract];
        func_vec.push(|a, b| -> i32 { a * b });
    }

}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn substract(a: i32, b: i32) -> i32 {
    return a + b;
}

fn multiply(a: i32, b: i32) -> i32 {
    return a + b;
}
fn based_on_flag(f: bool) -> Box<dyn Fn(i32) -> i32> {
    // The function should be stored in the heap memory?
    if f {
        return Box::new(|x: i32| x + 1);
    }
    return Box::new(|x: i32| x + 2);
}

// less binary but runtime burden
fn Calc1(a: i32, b: i32, func: Box<dyn Fn(i32, i32) -> i64>) -> i64 {
    func(a, b)
}

fn Calc2(a: i32, b: i32, func: impl Fn(i32, i32) -> i64) -> i64 {
    func(a, b)
}

fn Calc3(a: i32, b: i32, func: fn(i32, i32) -> i64) -> i64 {
    func(a, b)
}

// If the comlier does not know what to be created or executed at compile time, it cannot be
// allocared on the stack..
// It has to allocated on the heap --> VTables work on heap allocations.

// fn --> function pointer
// impl block --> implement at compile time..

// some times what to be called is decided at runtime


// Create an array --> add three functions in the array 
// should be able to add three different types of functions to the array.

// array[greet,sq,add] or vec

/*
greet(){
println!("Hello World!")
}
sq(i:i32)->i64{
return (i*i)as i64;
}
add(a:i32,b:i32)->i64{
return (a+b)as i64;
}
*/