// filepath: /d:/KGK/Job/Codings/Rust/Rust_dev/src/main.rs

use serde::{Serialize, Deserialize};
use serde_json;
use serde_yaml;
// use serde_xml_rs;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_employed: bool,
}

fn main() {
    // Create an instance of the struct
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        is_employed: true,
    };

    // Serialize to JSON
    let json_string = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_string);

    // Deserialize from JSON
    let deserialized_person: Person = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized JSON: {:?}", deserialized_person);

    // Serialize to YAML
    let yaml_string = serde_yaml::to_string(&person).unwrap();
    println!("Serialized YAML: {}", yaml_string);

    // Deserialize from YAML
    let deserialized_person: Person = serde_yaml::from_str(&yaml_string).unwrap();
    println!("Deserialized YAML: {:?}", deserialized_person);

    // // Serialize to XML
    // let xml_string = serde_xml_rs::to_string(&person).unwrap();
    // println!("Serialized XML: {}", xml_string);

    // // Deserialize from XML
    // let deserialized_person: Person = serde_xml_rs::from_str(&xml_string).unwrap();
    // println!("Deserialized XML: {:?}", deserialized_person);

    // Serialize to TOML
    let toml_string = toml::to_string(&person).unwrap();
    println!("Serialized TOML: {}", toml_string);

    // Deserialize from TOML
    let deserialized_person: Person = toml::from_str(&toml_string).unwrap();
    println!("Deserialized TOML: {:?}", deserialized_person);
}