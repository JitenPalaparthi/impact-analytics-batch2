static G:i32 =100; // The life of G lives as long as the application exists
fn main() {
   let mut x = &100;
   //{
    let y = 1000;
     x = &y;
    //drop(y);
   //}
   println!("{}",x);

   
   let s1 = "hello world".to_string();
   let l = get_length(&s1);

   let strs = ("Hello World. How are you doing!","Hello Impact Analytics");
   let s= max_str(&strs.0, &strs.1);

   println!("address of strs.0:{:p} strs.1:{:p}",(strs.0).as_ptr(),(strs.1).as_ptr());
   println!("address of ret s:{:p} ",s.as_ptr());

   let l=max_len(&strs.0,&strs.1);

   let str1="Hello Impact Analytics".to_string();

   let sub= find_word(&str1);

   println!("{}",sub);

//    let mut b = &mut 100;

//     *b = 200;
}

// Life means you are adivsing the borrow checker(part of the compiler),
// when it is confused about the life of the variable
// There is a concept calld Life time elision

fn get_length<'a>(s:&'a str)->i32{
    return s.len() as i32
}

fn get_str(s:&mut String)->&str{// elision 
    s.push_str("Done");
    return s
}

fn max_str<'a>(s1:&'a str ,s2:&'a str)->&'a str{
    if s1.len()>s2.len(){
        return s1 ;
    }
    s2
}

// Copy trait
fn max_len(s1:&str ,s2:&str)->usize{ // not going to return any reference
    if s1.len()>s2.len(){
        return s1.len()
    }
    s2.len()
}

// Create s1 and s2 in different scopes.. 
// Call the function and check what would happen

// Hello World // String
// Iterate the string byte by byte 
// s1 = "Hello World!"  --> ASCII and ' ' ascii 

// ' ' byte array .. 
// return the slice of that string s1[0..6]

fn find_word<'a>(s:&'a str)->&'a str{
    let buf: &[u8] = s.as_bytes();
    //let chars = s.chars();
    for (index,cbyte) in buf.iter().enumerate(){
        if *cbyte == b' '{
            return &s[0..index];
        }
    }
    s
    // let mut v=0;
    // for bt in buf{
    //     if *bt == b' '{
    //         return &s[0..v];
    //     }
    //     v+=1;
    // }
    //s
}