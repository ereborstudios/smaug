use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

#[get("/ping")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
