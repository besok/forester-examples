use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum_server::Handle;
use forester_rs::runtime::args::RtValue;
use forester_rs::runtime::env::daemon::context::DaemonContext;
use forester_rs::runtime::env::daemon::{AsyncDaemonFn, DaemonFn, StopFlag};
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;


pub struct HttpListener;

/// This is a simple daemon that listens to HTTP requests
/// and places the fields from http request to the blackboard.
impl AsyncDaemonFn for HttpListener {
    fn prepare(
        &mut self,
        ctx: DaemonContext,
        signal: CancellationToken,
    ) -> Pin<Box<dyn Future<Output=()> + Send>> {
        Box::pin(
            async move {
                let routing = Router::new()
                    .route("/", get(|| async { "OK" }))
                    .route("/action", post(handler))
                    .with_state(ctx)
                    .into_make_service_with_connect_info::<SocketAddr>();

                axum_server::bind(SocketAddr::from(([127, 0, 0, 1], 10000)))
                    .handle(stop_srv(signal))
                    .serve(routing)
                    .await
                    .unwrap();
            }
        )
    }
}

/// the handler that handles the request from Forester to stop the server.
fn stop_srv(signal: CancellationToken) -> Handle {
    let h = Handle::new();
    let handle = h.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = signal.cancelled() => {
                    h.shutdown();
                    return;
                }
                _ = tokio::time::sleep(
                    std::time::Duration::from_millis(5)
                ) => {}
            }
        }
    });
    handle
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Req {
    proceed: bool,
    threshold: usize,
}

async fn handler(
    State(ctx): State<DaemonContext>,
    Json(req): Json<Req>,
) -> impl IntoResponse {
    // place the fields from http request to the blackboard.
    ctx.bb
        .lock()
        .unwrap()
        .put("proceed".to_string(), RtValue::Bool(req.proceed), )
        .unwrap();

    ctx.bb
        .lock()
        .unwrap()
        .put("threshold".to_string(), RtValue::int(req.threshold as i64))
        .unwrap();

    // mark the fields as new and not stale.
    ctx.bb
        .lock()
        .unwrap()
        .put("stale".to_string(), RtValue::Bool(false))
        .unwrap();

    (StatusCode::OK)
}