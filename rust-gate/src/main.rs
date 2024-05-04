mod load_balancer;
pub mod service;
pub mod cache;

use actix_web::{web, App, HttpRequest, HttpServer};
use std::sync::{Arc, RwLock};
use load_balancer::{LoadBalancer, handle_request};
use service::Service;
use std::fs;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let services = create_services();
    let load_balancer = Arc::new(RwLock::new(LoadBalancer::new(services)));
    let cache: Arc<RwLock<cache::Cache<String, String>>> = Arc::new(RwLock::new(cache::Cache::new()));

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        let load_balancer = load_balancer.clone();
        let cache = cache.clone();
        App::new().route("/{service}/{endpoint}", web::get().to(move |_req: HttpRequest, path: web::Path<(String, String)>| {
            let load_balancer = load_balancer.clone();
            let cache = cache.clone();
            async move { handle_request(load_balancer, cache, path).await }
        }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn create_services() -> Vec<Service> {
    match env::var("DOCKER_ENV") {
        Ok(_) => {
            let services = fs::read_to_string("/usr/local/bin/services.json").expect("Unable to read services.json");
            serde_json::from_str(&services).expect("Unable to parse services.json")
        },
        Err(_) => {
            let services = fs::read_to_string("services.json").expect("Unable to read services.json");
            serde_json::from_str(&services).expect("Unable to parse services.json")
        }
    }
}