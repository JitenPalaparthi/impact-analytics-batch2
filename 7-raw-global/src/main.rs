static mut GLOBAL_COUNT:i32= 100; // DATA SEGMENT --> Lifetime is call the static lifetime
// Global variables are by design unsafe, may be due to data races and other memory related issues
#[allow(static_mut_refs)]
fn main() {
    {
        let mut company_name = "Impact Analytics"; // What is the lifetime of the variable
        company_name="Impact Analytics Inc";
        println!("{:p} {:p}",&company_name,company_name.as_ptr())
    }
        for i in 1..=10{
            
        unsafe{ // Rust cannot take care of this variable memory
            GLOBAL_COUNT+=1;
            println!("{}",GLOBAL_COUNT);// earlier versions of rust , you can directly 
            }
        }

        let tuple_vals = ("Impact Analytics",12312312.123,true);
        println!("{:#?}",tuple_vals);// you can print it as a debug print
    
    let mut s1:String  = "Hello World".to_string(); 

    let l= add_str(&mut s1, "! How are you doing?");

    // let mut s1:&mut str = "Hello World";
    // let l= add_str(s1, "! How are you doing?");

    let mut s1:&str="hello World";
    s1 = "Hello Impact Analytics";
    
    }

    fn add_str(s :&mut str,st:&str)->i32{
        s.to_string().push_str(st);
        return s.len() as i32;
    }
