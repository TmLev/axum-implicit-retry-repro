use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use axum::routing::get;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    setup_tracing();
    build_and_run().await
}

fn setup_tracing() {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            tracing_tree::HierarchicalLayer::new(2)
                .with_targets(true)
                .with_bracketed_fields(true),
        )
        .init();
}

#[tracing::instrument]
async fn build_and_run() {
    let router = Router::new()
        .route("/", get(move || async {
            // Sleep for 2 seconds...
            tokio::time::sleep(Duration::from_secs(2)).await;
            "Hello, axum!"
        }))
        // ... but timeout after 1 second.
        .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(1)))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = "0.0.0.0:9999".parse().unwrap();
    tracing::info!("Listening on http://{:?}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
