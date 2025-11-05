fn main() {
  
  let ref_num: &mut i32 = &mut 100;

  *ref_num = 500;

  let mut i = 100;
  let j=&mut i; //--?
  *j = 500;
  
  let str1 = "Hello World".to_string();
  let str2: &String = &str1;
  // temp means how long?
  println!("{}",str1);
  println!("{}",str2);
  
  let r= get_length(&str1);
  println!("{}",r);
  println!("{}",str1);

  let str3="Hello World";
  let r= get_length(str3);
  println!("{}",r);

}
fn get_length(s:&str)->usize{
    return s.len()
}