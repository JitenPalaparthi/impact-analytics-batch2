fn main() {
    let str1 = "hello world".to_string();
    let str2 = str1; // 
    // drop(str1) // Heap allocation
    // {
    //     let a = 100;
    //     let b = 200;
    //     {
    //         let c = a + b;
    //         println!("c={}", c);
    //     }
    //     println!("c={}", c);
    // }
    // println!("{} {}",a,b);
    let i = 100;
    let j = i; // There is no transfer or move of the owenership but it is a Copy
    // There is no ownere transfer bcz Copy trait has been implemented for primitive types

    // Borrowing rules
    let mut str1 = "Hello Impact Analytics".to_string(); // redeclared /recreated
    {
        let str2 = &str1;
        let str3 = &str1;
        println!("{} {}", str2, str3);
    }

    {
        let str4 = &mut str1;
        let str5 = &mut str1;
        // {
        // str4.push_str(". Doing some training");
        str5.push_str(" Learning Rust");
        // str4.push_str(". Doing some training");
    }

    // rule 3

    let str6 = &str1;
    let str8 = &str1;
    let str9 = &str1;

    // let str7 =&mut str1;

    //str7.push('!');
    println!("{} {} {}", str6, str8, str9);


    let (v,c)=get_char_count_by_str(&mut str1,".Trying to Learn Rust");

    println!("Vovels:{} Consonents & Special:{}",v,c);

    let counts=get_char_count_by_str(&mut str1,".Rust seems little tricky");
   
    println!("Vovels:{} Consonents & Special:{}",counts.0,counts.1);

    println!("{}",str1);
    // drop(str1)
}

// 1. Each value in rust has only one owner
// 2. If the owner goes out of the scope the variable is dropped
// 3. There are rules for Copy , and Clone (Generally Copy is implemented for all stack allocations)

// Borrowing Rules

// 0. Borrowing is always temparory
// 1. There can be any number of immutable borrows or references
// 2. Or there can be only one mutable borrow for a scope
// 3. There cannot be mutable reference while immutable one exists [Based on the versions of the rust]

// The result is. 1. To avoid references races during concurrent operations
// 2. To give clearcut idea to the compiler when to drop the original owners
// Owned Type vs Borrowed Type

fn get_char_count_by_str(s: &mut String, st: &str) -> (i32, i32) {
    s.push_str(st);
    let mut counts = (0, 0);
    for ch in s.chars() { // immutable char
        if ch=='A' || ch=='a' || ch=='E' || ch=='e' || ch=='I' || ch=='i'|| ch=='O' || ch=='o' || ch=='U' || ch=='u'{
            counts.0+=1;
        }else{
            counts.1+=1;
        }
    }
    return counts;
}
