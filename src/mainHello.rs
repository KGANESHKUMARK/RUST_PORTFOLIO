mod add;
// mod tests;

struct Person {
    name: String,
    age: u8,
    is_employed: bool,
}

fn main() {
    println!("Hello, world!");
    let result = add::add(2, 3); // Correctly call the add function from the add module
    println!("2 + 3 = {}", result);

    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        is_employed: true,
    };

    // Struct update syntax
    let person2 = Person {
        name: String::from("Bob"),
        ..person1
    };

    println!("Person 2: {}, {}, {}", person2.name, person2.age, person2.is_employed);
}