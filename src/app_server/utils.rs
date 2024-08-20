use std::time::Duration;

use crate::configs;
use axum::{
    extract::Query,
    http::{Error, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post, Router},
};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use tokio::signal;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::{self, methods::PLAYERS_API};

/// Initalize an Axum app server with the following features:
///
/// 1. Routes defined to serve the React SPA static files as well as server-side APIs
/// 2. Response compression
/// 3. Graceful shutdown (waits up to APP_SERVER_GRACEFUL_SHUTDOWN_MAX_DURATION seconds for in-flight requests to finish)
/// 4. Basic request and response logging
///
pub async fn init_app_server() -> Result<(), Error> {
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
        .unwrap();

    Ok(())
}

fn init_router() -> Router {
    // We need to serve from the build directory itself so the relative paths are correct for
    // the React app files.
    let binding =
        std::env::var("REACT_APP_DIST_DIR").expect("REACT_APP_DIST_DIR must be set in .env file.");
    let react_app_dist_location: &str = binding.as_str();
    let path_with_index_html = react_app_dist_location.to_string() + "/index.html";

    // Implement response compression
    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    Router::new()
        // Route to our React app
        // Mote tha fallback file is the SPA's root index.html, so that the server knows to send all browser click
        // to the SPA root app (which will then route from there) vs. looking for that phyiscal file to serve at that URL.
        .nest_service(
            "/",
            ServeDir::new(react_app_dist_location)
                .not_found_service(ServeFile::new(path_with_index_html)),
        )
        // Route to a random piece of dynamic generated content (simulating an API call/response)
        .route("/rando", get(random_number_handler))
        // Route to a random static html file
        .nest_service("/other-index", ServeFile::new("index2.html"))
        .route(PLAYERS_API, get(api::methods::get_players))
        .route(
            format!("{}{}", PLAYERS_API, "/:id").as_str(),
            get(api::methods::get_player),
        )
        .route(PLAYERS_API, post(api::methods::add_player))
        // .layer(middleware::from_fn(logging_middleware))
        .layer(comression_layer)
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(u64::from(
                configs::utils::get_env_var_as_number("APP_SERVER_GRACEFUL_SHUTDOWN_MAX_DURATION"),
            ))),
        ))
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Ivalid or malformed URL, please check and try again or report the issue.",
    )
}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Debug, Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn random_number_handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!(
        "<h1>Random Number: {}</h1></br><a href='/'>Go Back</a>",
        random_number
    ))
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
