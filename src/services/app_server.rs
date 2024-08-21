//! Provides utilities to initialize usage of the App Server and provide functions to interact with it.
//
use std::time::Duration;

use crate::api;
use crate::services::configs;
use axum::{http, response, Router};
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    decompression::RequestDecompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

/// Initalize an Axum app server with the following features:
///
/// 1. Routes defined to serve the Single Page Application (SPA) static files as well as API endpoints
/// 2. Response compression
/// 3. Graceful shutdown (waits up to APP_SERVER_GRACEFUL_SHUTDOWN_MAX_DURATION seconds for in-flight requests to finish)
/// 4. TODO SWY: Basic request and response logging
///
pub async fn init_app_server() -> Result<(), std::io::Error> {
    let app: axum::Router = init_router();

    let listener =
        tokio::net::TcpListener::bind(configs::get_env_var_or_panic("APP_SERVER_URL")).await?;

    tracing::debug!("App server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn init_router() -> Router {
    // Implement response compression
    let compression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let router = axum::Router::new()
        // Route for serving our Single Page Application (SPA)
        // Note tha fallback file is the SPA's root index.html, so that this server knows to send all url requests
        // (excpet where overridden later) to the SPA boostrap file which then handles everything from there.
        .nest_service(
            "/",
            ServeDir::new(
                // The SPA's build/distribution directory where all the compiled, static files reside
                configs::get_env_var_or_panic("SPA_DIST_DIR"),
            )
            .not_found_service(ServeFile::new(
                // The url for the core SPA bootstraping file
                configs::get_env_var_or_panic("SPA_BOOTSTRAP_URL"),
            )),
        )
        // TODO SWY: Example of a routing to a random static html file (something outside the SPA)
        .nest_service("/other-index", ServeFile::new("index2.html"))
        // .layer(axum::middleware::from_fn(logging_middleware))
        .layer(RequestDecompressionLayer::new())
        .layer(compression_layer)
        .layer((
            // Where request/response tracing/logging is declared
            // TODO SWY: Figure our why API calls are not logged, the SPA static files are
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(u64::from(
                configs::get_env_var_as_number_or_panic(
                    "APP_SERVER_GRACEFUL_SHUTDOWN_MAX_DURATION",
                ),
            ))),
        ))
        .fallback(handler_404);

    // Now add in all endpoints from our public APIs
    add_api_routes(router)
}

fn add_api_routes(mut router: Router) -> Router {
    let endpoints: Vec<api::endpoints::ApiEndpoint> = api::endpoints::get_all_endpoints();

    for endpoint in endpoints {
        router = router.route(endpoint.path.as_str(), endpoint.method_route);
    }
    router
}

async fn handler_404() -> impl response::IntoResponse {
    (
        http::StatusCode::NOT_FOUND,
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
