fn main() {
    let mut company_name1:&str = "Impact Analytics"; // string slice .. 
    let company_name2:String = String::from("Impact Analytics");
    let company_name3:&str = "Impact Analytics"; // string slice ..

    company_name1 = "你好 Impact Analytics"; 

    let mut company_name4: String= company_name1.to_string();// str to heap allocation
    println!("{} {} {} {}",company_name1,company_name2,company_name3,company_name4);
    let char1:char='你';
    company_name4.insert(0,char1);
    company_name4.insert_str(0,"hey how are you doing. ");
}

// &str vs String