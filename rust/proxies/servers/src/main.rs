use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let printer_app = Router::new()
        .route("/", get(print_root))
        .route("/print", post(print))
        .layer(TraceLayer::new_for_http());

    let logger_app = Router::new()
        .route("/", get(log_root))
        .route("/log", post(log))
        .layer(TraceLayer::new_for_http());

    let email_app = Router::new()
        .route("/", get(email_root))
        .route("/email", post(email))
        .layer(TraceLayer::new_for_http());

    let printer = async {
        axum::serve(
            TcpListener::bind("0.0.0.0:3000").await.unwrap(),
            printer_app,
        )
        .await
    };
    let logger =
        async { axum::serve(TcpListener::bind("0.0.0.0:3001").await.unwrap(), logger_app).await };
    let emailer =
        async { axum::serve(TcpListener::bind("0.0.0.0:3009").await.unwrap(), email_app).await };

    tokio::try_join!(printer, logger, emailer)?;

    Ok(())
}

async fn print_root() -> &'static str {
    tracing::info!("Printer: received a connection");
    "Hello from print-server\n"
}

async fn email_root() -> &'static str {
    tracing::info!("Email: received a connection");
    "Hello from email-server\n"
}

async fn log_root() -> &'static str {
    tracing::info!("Logger: received a connection");
    "Hello from log-server\n"
}

async fn print(Json(msg): Json<Message>) -> StatusCode {
    tracing::info!(text = msg.text, "Printing");

    StatusCode::CREATED
}

async fn log(Json(msg): Json<Message>) -> StatusCode {
    tracing::info!(text = msg.text, "Logging");

    StatusCode::CREATED
}

async fn email(Json(msg): Json<Message>) -> StatusCode {
    tracing::info!(text = msg.text, "Emailing");

    StatusCode::CREATED
}

#[derive(Deserialize)]
struct Message {
    text: String,
}
