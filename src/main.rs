mod database;
mod entities;
mod routes;
mod services;

use axum::Server;
use routes::routes;
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = routes();

    println!("Server run in {:?}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
