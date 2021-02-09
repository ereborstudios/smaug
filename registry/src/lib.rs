mod routes;

use actix_web::dev::Server;
use actix_web::App;
use actix_web::HttpServer;
use std::net::TcpListener;

pub fn app(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(routes::ping::index))
        .listen(listener)?
        .run();

    Ok(server)
}
