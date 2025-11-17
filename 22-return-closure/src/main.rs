fn main() {
    let result: fn(i32, i32) -> i64 = calc(); // result is a function, calc returns a functional pointer
    let r = result(10, 20);

    let result = multy_impl(25);
    let r = result(10);

    println!("r:{}", r);

    let func = |a: i32, b: i32| (a + b) as i64;

    let r = Calc1(10, 20, func);
    println!("r:{}", r);

    let r = Calc2(12, 14, |i: i32, j: i32| -> i64 {
        return (i + j) as i64;
    });
    println!("r:{}", r);

    let r = Calc1(12, 14, |i: i32, j: i32| -> i64 {
        return (i + j) as i64;
    });

    println!("r:{}", r);
    let func = |a: i32, b: i32| (a + b) as i64;
    let r = Calc2(12, 14, add);
    println!("r:{}", r);

    let mut f1 = make_counter();
    let r = f1();
    println!("r:{}", r);
     let r = f1();
    println!("r:{}", r);
     let r = f1();
    println!("r:{}", r);
     let r = f1();
    println!("r:{}", r);
}

fn add(a: i32, b: i32) -> i64 {
    return (a + b) as i64;
}

fn calc() -> fn(a: i32, b: i32) -> i64 {
    // It is a named function
    add // return a function pointer
}
// each function contains some kind of memory , it has an address

// there is clear difference between named function and closure.
// add is a named function

fn multy_impl(x: i32) -> impl Fn(i32) -> i32 { // compiler creates the code (impl)
    move |n: i32| n * x
}

// impl block would implment a function by the compiler at compile time

// Three traits

// Fn
// FnMut
// FnOnce

// Clone is implemented .. it automatically implement Copy

fn Calc1(a: i32, b: i32, func: fn(i32, i32) -> i64) -> i64 {
    func(a, b)
}

fn Calc2(a: i32, b: i32, func: impl Fn(i32, i32) -> i64) -> i64 {
    func(a, b)
}
// all the functions, whether closure or named function , must be evaluated by the compiler at compiler time.

fn make_counter()->impl FnMut()->i32{
    let mut counter=0; // Global variable --> function
    move ||->i32{
        counter+=1;
        counter
    }
}

// Fn , FnMut, FnOnce --> traits --> For trait bounds


fn fnonce_string()-> impl FnOnce(&str){
    let mut s1 = "".to_string();
    move|s|{
        s1.push_str(s);
    }
}