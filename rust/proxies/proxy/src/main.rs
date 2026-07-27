use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:4000").await?;
    let app = Router::new()
        .route("/", get(root))
        .route("/send", post(post_result));

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello from the proxy\n"
}

async fn post_result(Json(msg): Json<Message>) -> StatusCode {
    tracing::info!(msg = msg.text, "Proxying");
    let client = reqwest::Client::new();

    let print = client
        .post("http://0.0.0.0:3000/print")
        .json(&msg)
        .send()
        .await;
    tracing::info!(status=?print, "Printed");
    let log = client
        .post("http://0.0.0.0:3001/log")
        .json(&msg)
        .send()
        .await;
    tracing::info!(status=?log, "Logged");
    let email = client
        .post("http://0.0.0.0:3009/email")
        .json(&msg)
        .send()
        .await;
    tracing::info!(status=?log, "Emailed");

    let Ok(print) = print else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let Ok(log) = log else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let Ok(email) = email else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    if print.status() != StatusCode::CREATED {
        print.status()
    } else if log.status() != StatusCode::CREATED {
        log.status()
    } else {
        email.status()
    }
}

#[derive(Serialize, Deserialize)]
struct Message {
    text: String,
}
