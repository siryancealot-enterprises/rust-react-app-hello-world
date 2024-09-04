//! Provides utilities to initialize usage of the App Server and provide functions to interact with it.
//
use std::time::Duration;

use crate::api::endpoints;
use crate::services::configs;
use axum::http::StatusCode;
use axum::routing::{post, put};
use axum::{response, routing::get, Router};
use meilisearch_sdk::client::Client;
use sqlx::Postgres;
use tokio::net::TcpListener;
use tokio::signal::unix::SignalKind;
use tokio::signal::{self, unix};
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
/// 4. Basic request and response logging
///
pub async fn init_app_server(
    db_pool: sqlx::Pool<Postgres>,
    search_client: Client,
) -> Result<(), std::io::Error> {
    let app: axum::Router = init_router(db_pool, search_client);

    let listener = TcpListener::bind(configs::get_env_var_or_panic("APP_SERVER_URL")).await?;

    tracing::debug!("App server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Contains all shared state the Router will provide the handler of each request
#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::Pool<Postgres>,
    pub search_client: Client,
}

/// Initializes a [`axum::routing::Router`] with endpoint routes and other server runtime features
/// TODO SWY: This is only public to make it accessible for integrationt tests that need to boot up the app server with axum-test's approach
pub fn init_router(db_pool: sqlx::Pool<Postgres>, search_client: Client) -> Router {
    // Implement response compression
    let compression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let app_state: AppState = AppState {
        db_pool,
        search_client,
    };

    axum::Router::new()
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
        //
        //*** BEGIN: Add in all endpoints from our public APIs
        //
        // TODO SWY: We need to find a way to separate concerns with this section and have the endpoints returned
        // by ::api::endpoints and then built into routes in this function.
        .route(endpoints::PLAYERS_API, get(endpoints::get_players))
        .route(
            endpoints::build_id_path(endpoints::PLAYERS_API).as_str(),
            get(endpoints::get_player),
        )
        .route(endpoints::PLAYERS_API, put(endpoints::add_player))
        .route(
            endpoints::build_player_search_path().as_str(),
            post(endpoints::search_players),
        )
        //
        // *** END: Add in all endpoints from our public APIs
        //
        // Example of a routing to a random static html file (something outside the SPA)
        .nest_service("/other-index", ServeFile::new("index2.html"))
        // .layer(axum::middleware::from_fn(logging_middleware))
        .layer(RequestDecompressionLayer::new())
        .layer(compression_layer)
        .layer((
            // Where request/response tracing/logging is declared
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(u64::from(
                configs::get_env_var_as_number_or_panic(
                    "APP_SERVER_GRACEFUL_SHUTDOWN_MAX_DURATION",
                ),
            ))),
        ))
        // We add in the AppState which makes the DB conn pool and Search client (and future things) available to the method
        // handlers for our enddpoints added above.
        // see: https://mo8it.com/blog/sqlx-integration-in-axum/#states and https://docs.rs/axum/latest/axum/extract/struct.State.html
        .with_state(app_state)
        .fallback(handler_404)
}

async fn handler_404() -> impl response::IntoResponse {
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
        unix::signal(SignalKind::terminate())
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
