#![allow(unused)]

use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use axum::response::Html;

#[tokio::main]
async fn main(){
    let hello_router = Router::new().route(
        "/hello",
        get(|| async {Html ("Hello world")}),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTING on {addr} ");

    axum::Server::bind(&addr)
        .serve(hello_router.into_make_service())
        .await
        .unwrap();
}
