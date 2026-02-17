use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

pub async fn start_server() {
    let app = Router::new().route("/", get(|| async { "Lumina API" }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
