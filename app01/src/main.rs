use actix_web::{get, App, HttpServer, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Olá, Mundo! Bem-vindo ao primeiro servidor Rust!")
}

#[get("/sobre")]
async fn sobre() -> impl Responder {
    HttpResponse::Ok().body("Este é um servidor HTTP simples criado com Rust e Actix-web.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor iniciado em http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(sobre)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 