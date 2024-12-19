fn main() {
    let number = 7;

    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is exactly 5");
    } else {
        println!("The number is greater than 5");
    }


    if number % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
    
    match number {
        1 => println!("The number is 1"),
        2 | 3 | 5 | 7 | 11 => println!("The number is a prime number"),
        13..=19 => println!("The number is a teen"),
        _ => println!("The number is something else"),
    }

    if number % 2 == 0 {
        println!("The number is even");
    } else {
        match number {
            1 => println!("The number is 1"),
            2 | 3 | 5 | 7 | 11 => println!("The number is a prime number"),
            13..=19 => println!("The number is a teen"),
            _ => println!("The number is something else"),
        }
    }

    let direction = Direction::North;

    match direction {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East => println!("Heading East"),
        Direction::West => println!("Heading West"),
    }

    match read_file("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
    
}

enum Direction {
    North,
    South,
    East,
    West,
}

use std::fs::File;
use std::io::{self, Read};

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

