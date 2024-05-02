use std::sync::{Arc, RwLock};
use actix_web::{web, HttpResponse};
use reqwest;
use crate::service::Service;
use crate::cache::Cache;


pub struct LoadBalancer {
    services: Arc<RwLock<Vec<Service>>>,
    pub current_index: usize,
}

impl LoadBalancer {
    pub fn new(services: Vec<Service>) -> Self {
        Self {
            services: Arc::new(RwLock::new(services)),
            current_index: 0,
        }
    }

    pub fn find_service(&self, service_name: &str) -> Option<Service> {
        let services = self.services.read().unwrap();
        services
            .iter()
            .find(|service| service.name == service_name)
            .cloned()
    }
}

pub fn handle_request(
    load_balancer: Arc<RwLock<LoadBalancer>>,
    cache: Arc<RwLock<Cache<String, String>>>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let service_name = path.0.0.clone();
    let endpoint_name = path.0.1.clone();

    let mut load_balancer_guard = match load_balancer.write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let requested_service = match load_balancer_guard.find_service(&service_name) {
        Some(service) => service,
        None => return HttpResponse::NotFound().body("Service not found"),
    };

    if requested_service.enable_cache {
        if requested_service.cache_endpoints.contains(&endpoint_name) {
            let cache_guard = cache.read().unwrap();
            if let Some(cached_response) = cache_guard.get(&endpoint_name) {
                return HttpResponse::Ok().body(cached_response.clone());
            }
        }
    }

    let endpoint = requested_service.endpoints[load_balancer_guard.current_index].clone();
    load_balancer_guard.current_index = (load_balancer_guard.current_index + 1) % requested_service.endpoints.len();

    let client = reqwest::blocking::Client::new();
    let response = client.get(&format!("{}/{}", endpoint, endpoint_name)).send();

    match response {
        Ok(resp) => {
            let text = resp.text().unwrap_or_else(|_| "Error reading response".to_string());
            if requested_service.enable_cache {
                let mut cache_guard = cache.write().unwrap();
                cache_guard.insert(endpoint_name.clone(), text.clone());
            }
            HttpResponse::Ok().body(text)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error sending request"),
    }
}