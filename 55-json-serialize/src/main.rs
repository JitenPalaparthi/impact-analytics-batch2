use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
fn main() {
    let acc = HashMap::from([("icici bank".to_string(), "1123123".to_string())]);
    let emp1 = Employee {
        id: 101,
        name: "Jiten".to_string(),
        address: "Guntur,AP".to_string(),
        social_media: vec!["linkedin.com".to_string(), "instagram.com".to_string()],
        accounts: acc,
    };
    let result = serde_json::to_string(&emp1).unwrap();
    println!("Data of emp1:{}", result);
    let emp2: Employee = serde_json::from_str(&result).unwrap();
    println!("{:#?}", emp2);

    let emp_str = r#"
    {
    "id": 101,
    "name": "Jiten",
    "address": "Guntur AP",
    "social_media": [
        "linkedin.com",
        "instagram.com"
    ],
    "accounts": {
        "icici bank": "1123123"
    }
    }
    "#;

    let emp2: Employee = serde_json::from_str(emp_str).unwrap();
    println!("{:#?}", emp2);

    let emp3= json!({
    "id": 101,
    "name": "Jiten",
    "address": "Guntur AP",
    "social_media": [
        "linkedin.com",
        "instagram.com"
    ],
    "accounts": {
        "icici bank": "1123123"
    }
    });

    let id = &emp3["id"];
    let v = &emp3["accounts"];
    println!("\n---------");
    println!("id:{}",id);
    println!("accounts:{}",v);
    println!("bank:{}",&v["icici bank"]);
     println!("social media-1:{}",&emp3["social_media"][0])

}

#[derive(Debug, Serialize, Deserialize)] // this would generate the code
struct Employee {
    id: i32,
    name: String,
    address: String,
    social_media: Vec<String>,
    accounts: HashMap<String, String>,
}
