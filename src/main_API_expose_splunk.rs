use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use config::Config;
use reqwest::Client;

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
}

struct AppState {
    items: Mutex<Vec<Item>>,
    splunk_config: SplunkConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    address: String,
    port: u16,
}

#[derive(Deserialize, Clone)]
struct SplunkConfig {
    hec_url: String,
    hec_token: String,
}

async fn get_items(data: web::Data<AppState>) -> impl Responder {
    let items = data.items.lock().unwrap();
    HttpResponse::Ok().json(&*items)
}

async fn get_item(data: web::Data<AppState>, item_id: web::Path<u32>) -> impl Responder {
    let items = data.items.lock().unwrap();
    let item_id = item_id.into_inner();
    if let Some(item) = items.iter().find(|&item| item.id == item_id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

async fn create_item(data: web::Data<AppState>, new_item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    items.push(new_item.into_inner());
    send_to_splunk(&data.splunk_config, "Item created").await;
    HttpResponse::Created().body("Item created")
}

async fn update_item(data: web::Data<AppState>, item_id: web::Path<u32>, updated_item: web::Json<Item>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let item_id = item_id.into_inner();
    if let Some(item) = items.iter_mut().find(|item| item.id == item_id) {
        item.name = updated_item.name.clone();
        item.quantity = updated_item.quantity;
        send_to_splunk(&data.splunk_config, "Item updated").await;
        HttpResponse::Ok().body("Item updated")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

async fn delete_item(data: web::Data<AppState>, item_id: web::Path<u32>) -> impl Responder {
    let mut items = data.items.lock().unwrap();
    let item_id = item_id.into_inner();
    if items.iter().any(|item| item.id == item_id) {
        items.retain(|item| item.id != item_id);
        send_to_splunk(&data.splunk_config, "Item deleted").await;
        HttpResponse::Ok().body("Item deleted")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

async fn send_to_splunk(splunk_config: &SplunkConfig, event: &str) {
    let client = Client::new();
    let payload = serde_json::json!({
        "event": event,
    });

    let _ = client.post(&splunk_config.hec_url)
        .bearer_auth(&splunk_config.hec_token)
        .json(&payload)
        .send()
        .await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let mut settings = Config::default();
    settings.merge(config::File::with_name("config")).unwrap();
    let server_config: ServerConfig = settings.get::<ServerConfig>("server").unwrap();
    let splunk_config: SplunkConfig = settings.get::<SplunkConfig>("splunk").unwrap();

    let app_state = web::Data::new(AppState {
        items: Mutex::new(vec![]),
        splunk_config,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/items", web::get().to(get_items))
            .route("/items/{id}", web::get().to(get_item))
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind((server_config.address, server_config.port))?
    .run()
    .await
}