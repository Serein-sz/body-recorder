mod weight;

use std::net::SocketAddr;

use axum::{Json, Router, routing::post};
use crate::{error::AppResult};

pub async fn run(port: u16) -> AppResult<()> {
    let app = Router::new().route("/sync", post(handle_post));
   
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("监听 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn handle_post(Json(payload): Json<serde_json::Value>) {
    weight::sync_weight(payload).await.unwrap();
    // 打印收到的 JSON
    // println!("收到请求: {:?}", payload);
    
    // // 或者用 serde_json 美化打印
    // println!("美化打印:\n{}", serde_json::to_string_pretty(&payload).unwrap());
}