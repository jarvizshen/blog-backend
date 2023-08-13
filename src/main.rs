mod controller;
use actix_web::{App, HttpServer};
#[tokio::main]
async fn main() {
    HttpServer::new(|| App::new().service(controller::demo::hello))
        .bind(("0.0.0.0", 8080))
        .unwrap()
        .run()
        .await
        .unwrap()
}
