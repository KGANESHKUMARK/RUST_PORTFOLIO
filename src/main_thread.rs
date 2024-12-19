// use my_library::math_utils;
use std::thread;

fn main(){
    println!("Hello, World!");

    // let sum = math_utils::add(5, 3);
    // println!("Sum: {}", sum);

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread says: {}", i);
        }
    });

    // Main thread continues running
    for i in 1..5 {
        println!("Main thread says: {}", i);
    }

    handle.join().unwrap(); // Wait for the spawned thread to finish

}