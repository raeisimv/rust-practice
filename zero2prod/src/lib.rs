use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, HttpResponse};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscriptionFormData {
    name: String,
    email: String,
}

async fn subscriptions(body: web::Form<SubscriptionFormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
