#[derive(Debug)]
pub struct Address<'a> {
    // Any references are there as fields in the struct annotate with lifetime
    pub City: &'a str,
    pub Street: &'a str,
    pub PinCode: &'a str,
}

impl<'a> Address<'a> {
    pub fn new(addr: &'a str) -> Self {
        let mut city = "";
        let mut street = "";
        let mut pincode = "";
        for s in addr.split(';') {
            if let Some(c) = s.strip_prefix("City:") {
                city = c;
            }
            if let Some(c) = s.strip_prefix("Street:") {
                street = c;
            }

            if let Some(c) = s.strip_prefix("Pincode:") {
                pincode = c;
            }
        }
        Self {
            City: city,
            Street: street,
            PinCode: pincode,
        }
    }
}

// pub struct Address1{
//    pub City: String,
//    pub Street: String,
//    pub PinCide: String,
// }


pub fn split_addr(addr:&str)->Address{
     let mut city = "";
        let mut street = "";
        let mut pincode = "";
        for s in addr.split(';') {
            if let Some(c) = s.strip_prefix("City:") {
                city = c;
            }
            if let Some(c) = s.strip_prefix("Street:") {
                street = c;
            }

            if let Some(c) = s.strip_prefix("Pincode:") {
                pincode = c;
            }
        }
        Address {
            City: city,
            Street: street,
            PinCode: pincode,
        }
}