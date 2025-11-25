use std::{
    fs::File,
    io::{Result, Write},
};

fn main(){

   match write_to_file("log.txt", "Hello Impact Analytics Inc!") {
       Ok(())=>println!("Successfully written to file"),
       Err(e)=>println!("{}",e),
   } 
}

fn write_to_file(filename: &str, data: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
