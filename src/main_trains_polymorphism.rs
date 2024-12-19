


// fn main(){
//     println!("Hello, world!");
// }

trait Greet {
    fn greet(&self) -> String;

    //create same method name with different implementation 
    

    fn greet_twice(&self) -> String {
        self.greet() + " " + &self.greet()
    }

    fn greet_thrice(&self) -> String {
        self.greet() + " " + &self.greet_twice()
    }

}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}!", self.name)
    }

    fn greet_twice(&self) -> String {
        format!("Hello, my name is {}! It's nice to meet you!", self.name)
    }
    
}

// fn greet<T: Greet>(thing: T) {
//     println!("{}", thing.greet());
// }


fn main() {
    let person = Person {
        name: String::from("Alice"),
    };
    println!("{}", person.greet());
    println!("{}", person.greet_twice());
    println!("{}", person.greet_thrice());
}