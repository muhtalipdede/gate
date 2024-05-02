mod load_balancer;
pub mod service;
pub mod cache;

use actix_web::{web, App, HttpRequest, HttpServer};
use std::sync::{Arc, RwLock};
use load_balancer::{LoadBalancer, handle_request};
use service::Service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let service1 = Service {
        name: "service1".to_string(),
        endpoints: vec!["http://localhost:8081".to_string(), "http://localhost:8082".to_string()],
        enable_cache: true,
        cache_endpoints: vec!["health".to_string()],
        cache_duration: 10,
    };

    let service2 = Service {
        name: "service2".to_string(),
        endpoints: vec!["http://localhost:8083".to_string(), "http://localhost:8084".to_string()],
        enable_cache: true,
        cache_endpoints: vec![],
        cache_duration: 10,
    };

    let load_balancer = Arc::new(RwLock::new(LoadBalancer::new(vec![service1, service2])));
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}