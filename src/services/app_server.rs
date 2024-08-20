//! Provides utilities to initialize usage of the App Server and provide functions to interact with it.
//
use std::time::Duration;

use crate::api;
use crate::services::configs;
use axum::{http::StatusCode, response::IntoResponse, routing::Router};
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    decompression::RequestDecompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initalize an Axum app server with the following features:
///
/// 1. Routes defined to serve the Single Page Application (SPA) static files as well as API endpoints
/// 2. Response compression
/// 3. Graceful shutdown (waits up to APP_SERVER_GRACEFUL_SHUTDOWN_MAX_DURATION seconds for in-flight requests to finish)
/// 4. Basic request and response logging
///
pub async fn init_app_server() {
    // Enable tracing.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let app: Router = init_router();

    let listener = tokio::net::TcpListener::bind(
        std::env::var("APP_SERVER_URL").expect("APP_SERVER_URL must be set in .env file."),
    )
    .await
    .unwrap();

    println!(
        "App server listening on: {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("App server failed to initialize");
}

fn init_router() -> Router {
    // Implement response compression
    let compression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let router = Router::new()
        // Route for serving our Single Page Application (SPA)
        // Note tha fallback file is the SPA's root index.html, so that this server knows to send all url requests
        // (excpet where overridden later) to the SPA boostrap file which then handles everything from there.
        .nest_service(
            "/",
            ServeDir::new(
                // The SPA's build/distribution directory where all the compiled, static files reside
                std::env::var("SPA_DIST_DIR").expect("SPA_DIST_DIR must be set in .env file."),
            )
            .not_found_service(ServeFile::new(
                // The url for the core SPA bootstraping file
                std::env::var("SPA_BOOTSTRAP_URL")
                    .expect("SPA_BOOTSTRAP_URL must be set in .env file."),
            )),
        )
        // TODO SWY: Example of a routing to a random static html file (something outside the SPA)
        .nest_service("/other-index", ServeFile::new("index2.html"))
        // .layer(middleware::from_fn(logging_middleware))
        .layer(RequestDecompressionLayer::new())
        .layer(compression_layer)
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(u64::from(
                configs::get_env_var_as_number("APP_SERVER_GRACEFUL_SHUTDOWN_MAX_DURATION"),
            ))),
        ))
        .fallback(handler_404);

    // Now add in all endpoints from our public APIs
    api::endpoints::add_all_endpoints(router)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Ivalid or malformed URL, please check and try again or report the issue.",
    )
}

// Bind various ways to detect and listen for a shutdown command, which allows the graceful shutdown above.
// see: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

// Basic logging: logs a visit to any of this server's endpoints.
// Yes, "middleware" is a term in Rust and Axum:
// "Middleware in Rust refers to the concept of adding additional layers or functionality between different components of a software system,"
// https://docs.rs/axum/latest/axum/middleware/index.html
// https://medium.com/@alexeusgr/what-is-middleware-in-rust-43924cad8076
// async fn logging_middleware(req: Request<Body>, next: Next) -> Response {
//     println!("Received a request to {}, {}", req.uri(), req.method());
//     next.run(req).await
// }
