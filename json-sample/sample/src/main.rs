use std::error::Error;
use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::Path;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    phones: [String; 2],
}

fn main() {
    println!("json sample");

    // let txtdata = r#"{
    // "name": "John Doe",
    // "age": 43,
    // "phones": [
    //     "+44 1234567",
    //     "+44 2345678"
    // ]
    // }"#;
    // let v: serde_json::Value = serde_json::from_str(txtdata).unwrap();
    // println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    // let point = Point { x: 1, y: 2 };
    // let ser = serde_json::to_string(&point).unwrap();
    // println!("{}", ser);

    // let deser: Point = serde_json::from_str(&ser).unwrap();
    // println!("{:?}", deser);
    let person = Person {
        name: "John".to_string(),
        age: 43,
        phones: ["+44 1234567".to_string(), "+44 2345678".to_string()],
    };
    let txtjson = serde_json::to_string(&person).unwrap();

    let path = Path::new("out.json");
    let fout = match File::create(&path) {
        Err(why) => panic!(
            "couldn't create {}: {}",
            path.display(),
            Error::description(&why)
        ),
        Ok(file) => file,
    };

    let mut fout = BufWriter::new(fout);
    match fout.write(txtjson.as_bytes()) {
        Err(why) => panic!(
            "couldn't write to {}: {}",
            path.display(),
            Error::description(&why)
        ),
        Ok(_) => (),
    };
}
