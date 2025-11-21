
pub mod address;
use address::{Address,split_addr};

fn main() {
    let addr_str = "City:Bangalore;Street:Raj Street;Pincode:560086";
    let a1: Address<'_> = Address::new(addr_str);
    println!("{:?}",a1);
    println!("City:{} Street:{} PinCode:{}",a1.City,a1.Street,a1.PinCode);

    let addr_str = "City:Bangalore;Street:Raj Street;Pincode:560086";

    let a1= split_addr(addr_str);
    println!("City:{} Street:{} PinCode:{}",a1.City,a1.Street,a1.PinCode);
}

