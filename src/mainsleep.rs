use tokio::time::{sleep, Duration};

// Define an asynchronous function
async fn do_something() {
    println!("Doing something...");
    // Simulate an asynchronous operation with sleep
    sleep(Duration::from_secs(2)).await;
    println!("Done!");
}

#[tokio::main]
async fn main() {
    println!("Starting...");
    // Call the asynchronous function and await its result
    do_something().await;
    println!("Finished!");
}