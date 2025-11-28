use prost::Message;
mod pb {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/pb/employee.rs"));
}

use pb::Employee;
use std::collections::HashMap;
fn main() {
    let acc = HashMap::from([("icici bank".to_string(), "1123123".to_string())]);
    let emp1 = Employee {
        id: 101,
        name: "Jiten".to_string(),
        address: "Guntur,AP".to_string(),
        social_media: vec!["linkedin.com".to_string(), "instagram.com".to_string()],
        accounts: acc,
    };
    // serialize the data
    let mut buf: Vec<u8> = Vec::new();

    buf.reserve(emp1.encoded_len());
    emp1.encode(&mut buf).unwrap(); // byte foramt

    println!("encoded bytes:{:?}", &buf);
    // emp1.encode(buf)

    // deserialize
    let emp = Employee::decode(&*buf).unwrap();
    println!("Employee after desecialize:{:#?}", emp);
}
