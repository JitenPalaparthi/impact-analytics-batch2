use std::collections::HashMap;

fn main() {
    // SHA --> It is not to encrypt or decrypt

    let mut map: HashMap<String, i32>= HashMap::new();

   map.insert("Bangalore-1".to_string(), 560086);
   map.insert("Bangalore-2".to_string(), 560096);
   map.insert("Bangalore-3".to_string(), 560034);


   for (k,v) in &map{
        println!("Key:{} Value:{}",k,v);
   }

  // println!("{}",map)

}
