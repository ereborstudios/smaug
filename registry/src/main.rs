pub mod index;

use actix_web::get;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Index")
}
