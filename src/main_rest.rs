use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    body: String,
    userId: u32,
}

async fn get_request(client: &Client) -> Result<(), Box<dyn Error>> {
    let response = client.get("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?
        .json::<Post>()
        .await?;
    println!("GET response: {:?}", response);
    Ok(())
}

async fn post_request(client: &Client) -> Result<(), Box<dyn Error>> {
    let new_post = Post {
        id: 101,
        title: String::from("foo"),
        body: String::from("bar"),
        userId: 1,
    };

    let response = client.post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json::<Post>()
        .await?;
    println!("POST response: {:?}", response);
    Ok(())
}

async fn put_request(client: &Client) -> Result<(), Box<dyn Error>> {
    let updated_post = Post {
        id: 1,
        title: String::from("updated title"),
        body: String::from("updated body"),
        userId: 1,
    };

    let response = client.put("https://jsonplaceholder.typicode.com/posts/1")
        .json(&updated_post)
        .send()
        .await?
        .json::<Post>()
        .await?;
    println!("PUT response: {:?}", response);
    Ok(())
}

async fn delete_request(client: &Client) -> Result<(), Box<dyn Error>> {
    let response = client.delete("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;
    println!("DELETE response status: {}", response.status());
    Ok(())
}

async fn head_request(client: &Client) -> Result<(), Box<dyn Error>> {
    let response = client.head("https://jsonplaceholder.typicode.com/posts/1")
        .send()
        .await?;
    println!("HEAD response status: {}", response.status());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    get_request(&client).await?;
    post_request(&client).await?;
    put_request(&client).await?;
    delete_request(&client).await?;
    head_request(&client).await?;
    
    Ok(())
}