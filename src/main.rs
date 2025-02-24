/*
    SmartsHub Content Delivery Service
    A microservice that serves files from a local directory to users throught a HTTP API built on Rust.
*/
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use sha1::{Digest, Sha1};
use std::env;
use std::path::Path;
use tokio::fs;

async fn get_file(file: web::Path<String>) -> Result<HttpResponse> {
    let filename = file.into_inner();
    
    // Prevent path traversal attacks
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Ok(HttpResponse::BadRequest().body("Invalid filename"));
    }

    let path = Path::new("resources").join(&filename);
    match fs::read(&path).await {
        Ok(content) => Ok(HttpResponse::Ok().body(content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(HttpResponse::NotFound().body("File not found"))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn get_sha1(file: web::Path<String>) -> Result<HttpResponse> {
    let filename = file.into_inner();
    
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Ok(HttpResponse::BadRequest().body("Invalid filename"));
    }

    let path = Path::new("resources").join(&filename);
    match fs::read(&path).await {
        Ok(content) => {
            let mut hasher = Sha1::new();
            hasher.update(&content);
            let result = hasher.finalize();
            Ok(HttpResponse::Ok().body(format!("{:x}", result)))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(HttpResponse::NotFound().body("File not found"))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn get_size(file: web::Path<String>) -> Result<HttpResponse> {
    let filename = file.into_inner();
    
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Ok(HttpResponse::BadRequest().body("Invalid filename"));
    }

    let path = Path::new("resources").join(&filename);
    match fs::metadata(&path).await {
        Ok(meta) => Ok(HttpResponse::Ok().body(meta.len().to_string())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(HttpResponse::NotFound().body("File not found"))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/file/{file}")
                    .route(web::get().to(get_file)),
            )
            .service(
                web::resource("/file/{file}/sha1")
                    .route(web::get().to(get_sha1)),
            )
            .service(
                web::resource("/file/{file}/size")
                    .route(web::get().to(get_size)),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}