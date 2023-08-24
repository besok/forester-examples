use axum::extract::ConnectInfo;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Json, Router, ServiceExt};
use forester_http::client::{ForesterHttpClient, TickError};
use forester_http::{ForesterRemoteAction, RemoteActionRequest, TickResult};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let routing = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/calculate", post(handler))
        .into_make_service();

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 10000)))
        .serve(routing)
        .await
        .unwrap();
}

async fn handler(Json(req): Json<RemoteActionRequest>) -> impl IntoResponse {
    let client = ForesterHttpClient::new(req.serv_url.clone());
    client
        .put("calculated".to_string(), json!(true))
        .await
        .unwrap();

    client.lock("calculated".to_string()).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    client.unlock("calculated".to_string()).await.unwrap();

    client
        .new_trace_event(req.tick, "Calculated".to_string())
        .await
        .unwrap();

    (StatusCode::OK, Json::from(TickResult::Success))
}
