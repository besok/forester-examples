use axum::extract::ConnectInfo;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Json, Router, ServiceExt};
use forester_http::client::{ForesterHttpClient, TickError};
use forester_http::{ForesterRemoteAction, RemoteActionRequest, TickResult};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routing = Router::new()
        .route("/", get(|| async { "OK" }))
        .route("/action", post(handler))
        .into_make_service_with_connect_info::<SocketAddr>();

    axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 10000)))
        .serve(routing)
        .await
        .unwrap();
}
async fn handler(Json(req): Json<RemoteActionRequest>) -> impl IntoResponse {
    let url = req.clone().serv_url;

    let client = ForesterHttpClient::new(url);
    let trace = client
        .print_trace()
        .await
        .unwrap()
        .text()
        .await
        .unwrap_or_default();

    println!("trace is {}", trace);

    let result = client
        .put("test".to_string(), json!({"f1":1, "f2":2, "f3":3}))
        .await;
    println!("result of putting {:?}", result);

    let result = client
        .get("test".to_string())
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap_or_default();

    println!("result of getting {:?}", result);

    client.lock("test".to_string()).await.unwrap();

    (StatusCode::OK, Json::from(RemoteAction.tick(req)))
}

struct RemoteAction;

impl ForesterRemoteAction for RemoteAction {
    fn tick(&self, request: RemoteActionRequest) -> TickResult {
        TickResult::Success
    }
}
