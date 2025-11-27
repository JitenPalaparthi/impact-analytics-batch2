use std::ffi::CString;
use std::os::raw::{c_char, c_int};

unsafe extern "C" { // i am using external c libraries
    // Must match the signatures in mylib.h / mylib.c
    fn add_ints(a: c_int, b: c_int) -> c_int;
    fn greet(name: *const c_char);
}

fn main() {
    unsafe {
        // 1. Call add_ints
        let a: c_int = 10;
        let b: c_int = 32;
        let sum = add_ints(a, b);
        println!("Result from C add_ints({a}, {b}) = {sum}");

        // 2. Call greet with a C string
        let name = CString::new("Jiten").expect("CString::new failed");
        greet(name.as_ptr());
    }

    println!("Back in Rust main()");
}