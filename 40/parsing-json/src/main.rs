extern crate serde_json;
extern crate serde;
 #[macro_use]
extern crate serde_derive;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool
}

fn main() {
    let json_str = r#"
        {
            "name": "Mehmet",
            "age": 22,
            "is_male": true
        }
    "#;

    let res  = serde_json::from_str(json_str);

    if res.is_ok(){
        //let p: JsonValue = res.unwrap();
        let p: Person = res.unwrap();
        //println!("the name is {}", p["name"]);
        //println!("the name is {}", p["name"].as_str().unwrap());
        println!("the name is {}", p.name);
        println!("the name is {}", p.age);
        println!("the name is {}", p.is_male);
    } else{
        println!("sorry... couldn't parse json :(  ");
    }
}
