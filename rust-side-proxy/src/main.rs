use std::{cell::Cell, io::Write};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use reqwest;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <sidecar_port> <service_port>", args[0]);
        std::process::exit(1);
    }

    let sidecar_port: u16 = args[1].parse().expect("Invalid sidecar port");
    let service_port: u16 = args[2].parse().expect("Invalid service port");

    println!("Sidecar proxy running at http://0.0.0.0:{}", sidecar_port);
    println!("Service running at http://0.0.0.0:{}", service_port);

    HttpServer::new(move || {
        App::new().route("/{path:.*}", web::get().to(move |req, path| {
            let service_port = web::Data::new(Cell::new(service_port));
            handle_request(req, service_port, path)
        }))
    })
    .bind(("0.0.0.0", sidecar_port))?
    .run()
    .await
}

async fn handle_request(req: HttpRequest, service_port: web::Data<Cell<u16>>, _path: web::Path<String>) -> HttpResponse {
    let method = req.method();
    let path = req.match_info().query("path");
    let url = format!("http://host.docker.internal:{}/{}", service_port.get(), path);

    println!("{} {}", method, url);

    let origin: String = match req.headers().get("origin") {
        Some(value) => value.to_str().unwrap_or_default().to_string(),
        None => String::new(),
    };

    let correlation_id: String = match req.headers().get("correlation-id") {
        Some(value) => value.to_str().unwrap_or_default().to_string(),
        None => String::new(),
    };

    println!("Correlation ID: {}", correlation_id);
    
    let client = reqwest::blocking::Client::new();
    let response = client.get(&url).send();

    if &origin != "" {
        println!("Origin: {}", origin);
    }


    match response {
        Ok(resp) => {
            let text = resp.text().unwrap_or_else(|_| "Error reading response".to_string());
            HttpResponse::Ok().body(text)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error sending request"),
    }
}