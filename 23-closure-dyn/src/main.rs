fn main() {
    let b1 = based_on_flag(true);

    let r = b1(100);
    println!("r:{}", r);

    let b1 = based_on_flag(false);

    let r = b1(100);
    println!("r:{}", r);
}

fn based_on_flag(f: bool) -> Box<dyn Fn(i32) -> i32> {
    // The function should be stored in the heap memory?
    if f {
        return Box::new(|x: i32| x + 1);
    }
    return Box::new(|x: i32| x + 2);
}

// less binary but runtime burden
fn Calc1(a: i32, b: i32, func: Box< dyn Fn(i32, i32) -> i64>) -> i64 {
    func(a, b)
}

fn Calc2(a: i32, b: i32, func: impl Fn(i32, i32) -> i64) -> i64 {
    func(a, b)
}

fn Calc3(a: i32, b: i32, func:  fn(i32, i32) -> i64) -> i64 {
    func(a, b)
}



// If the comlier does not know what to be created or executed at compile time, it cannot be
// allocared on the stack..
// It has to allocated on the heap --> VTables work on heap allocations.

// fn --> function pointer
// impl block --> implement at compile time..

// some times what to be called is decided at runtime
