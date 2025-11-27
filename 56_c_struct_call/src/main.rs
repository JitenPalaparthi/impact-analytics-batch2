use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct Person {
    pub name: *const c_char,
    pub age: c_int,  
}

unsafe extern "C" {
    // From mylib.h / mylib.c

    // Point-related
    fn make_point(x: c_int, y: c_int) -> Point;
    fn manhattan_distance(p: *const Point) -> c_int;

    // Person-related
    fn make_person(name: *const c_char, age: c_int) -> Person;
    fn print_person(p: *const Person);
    fn birthday(p: *mut Person) -> c_int;
}

fn main() {
    unsafe {
        // ---------- Using Point struct ----------
        let p = make_point(10, -4);
        println!("Rust: Point from C = {:?}", p);

        let dist = manhattan_distance(&p as *const Point);
        println!("Rust: manhattan_distance = {}", dist);

        // ---------- Using Person struct ----------
        // Keep the CString alive as long as the Person uses it
        let name_c = CString::new("Jiten").expect("CString::new failed");

        let mut person = make_person(name_c.as_ptr(), 40);
        println!("Rust: Person from C = {:?}", person);

        print_person(&person as *const Person);

        let new_age = birthday(&mut person as *mut Person);
        println!("Rust: After birthday, age from C = {}", new_age);

        print_person(&person as *const Person);
    }

    println!("Back in Rust main()");
}