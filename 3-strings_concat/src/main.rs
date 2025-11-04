fn main() {
   let s1: &str = "Impact "; // not owned type
   let s2 = "Analytics"; // not owned type
   let s3 = s1.to_owned() + s2;
   println!("{}",s3);

   let s1:String = String::from("Impact ");
   let s2:String = String::from("Analytics");
   let s3 = s1 + &s2;
   println!("{}",s3);
}
