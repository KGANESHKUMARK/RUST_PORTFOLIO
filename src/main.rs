use log::{info, warn, error, debug, trace};
use env_logger;
use redis::AsyncCommands;
use tokio;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    env_logger::init();
    println!("Hello, world!");
    info!("Hello, world info!");
    warn!("Hello, world warn!");
    error!("Hello, world error!");
    debug!("Hello, world debug!");
    trace!("Hello, world trace!");

    // Connect to the Redis server
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    // Set a key-value pair
    let _: () = con.set("my_key", "my_value").await?;

    // Get the value of the key
    let value: String = con.get("my_key").await?;
    println!("The value of 'my_key' is: {}", value);

    Ok(())
}