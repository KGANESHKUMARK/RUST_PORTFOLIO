use serde::{Serialize, Deserialize};
use serde_json;


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

    // Serialize the struct to a JSON string
    let json_string = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_string);

    // Deserialize the JSON string back to a struct
    let deserialized_person: Person = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized struct: {:?}", deserialized_person);
}