# Rust Memory & Collections Talk Series – With Examples & “Solutions”

Below are the same 20 talks, now with concrete code examples and short “solutions” / explanations you can show while teaching.

---

## Talk 1: Mental Model of Ownership & Borrowing

### Example

```rust
fn main() {
    let s1 = String::from("hello");

    // Ownership moves into s2
    let s2 = s1;

    // println!("{}", s1); // ❌ error: borrow of moved value

    // Correct: use s2
    println!("s2 = {}", s2);

    // If you actually need two owned values:
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
}
```

### Explanation

- `s1` is moved into `s2`; `s1` is no longer valid.
- Clone allocates a new String on the heap, so you get two independent owners.

---

## Talk 2: Borrow Checker – Friend, Not Enemy

### Example

```rust
fn main() {
    let mut x = 10;

    let r1 = &x;          // immutable borrow
    println!("r1 = {}", r1);

    // let r2 = &mut x;   // ❌ cannot borrow `x` as mutable because it is also borrowed as immutable

    // r1 is no longer used after this line
    let r2 = &mut x;      // ✅ now allowed due to non-lexical lifetimes
    *r2 += 5;
    println!("x = {}", x);
}
```

### Explanation

- You can have many `&T` or exactly one `&mut T` at a time.
- Because `r1` is not used after its last read, Rust can end its lifetime early and allow `r2`.

---

## Talk 3: Lifetimes – Who Outlives Whom?

### Example

```rust
// returns the longer of two string slices
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let a = String::from("short");
    let b = String::from("much longer");

    let result = longest(&a, &b);
    println!("Longest is: {}", result);
}
```

### Explanation

- `'a` ties the lifetimes of `s1`, `s2`, and the returned `&str`.
- The return reference is guaranteed not to outlive the shorter of the two inputs.

---

## Talk 4: Box Basics – Owning Data on the Heap

### Example

```rust
fn main() {
    let x = 42;
    let boxed = Box::new(x); // move 42 into heap

    println!("boxed value = {}", boxed);  // Deref coercion at work

    // Box moves like a normal value
    let boxed2 = boxed;
    // println!("{}", boxed); // ❌ moved

    println!("boxed2 value = {}", boxed2);
}
```

### Explanation

- `Box::new` puts the data on the heap but ownership is still tracked by a stack variable (`boxed`).
- Box implements `Deref`, so `println!` can treat it like `&T`.

---

## Talk 5: Box, Drop & Custom Smart Pointer

### Example: simple logging smart pointer

```rust
use std::ops::{Deref, DerefMut};

struct MyBox<T> {
    inner: T,
}

impl<T> MyBox<T> {
    fn new(inner: T) -> Self {
        println!("MyBox::new");
        Self { inner }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}

fn main() {
    let mut v = MyBox::new(vec![1, 2, 3]);
    v.push(4);
    println!("v = {:?}", *v);
} 
```

### Explanation

- `Deref`/`DerefMut` allow `*v` and method calls on `Vec` to work.
- `Drop` runs automatically when `v` goes out of scope.

---

## Talk 6: References & Raw Pointers

### Example

```rust
fn main() {
    let mut x = 10;

    let r: &mut i32 = &mut x;         // safe mutable reference
    *r += 1;

    let p: *mut i32 = r as *mut i32;  // raw pointer from reference

    unsafe {
        *p += 1;  // unsafe: dereferencing raw pointer
    }

    println!("x = {}", x);
}
```

### Explanation

- `&mut i32` is checked by the borrow checker.
- `*mut i32` is a raw pointer; dereferencing it requires `unsafe` and no safety guarantees from the compiler.

---
... (truncated for brevity)
