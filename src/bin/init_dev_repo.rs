//! A utility for initalizing repos on developer or test environments.
use colored::Colorize;
use rust_react_app_hello_world::{environment_utils::dev_and_test_utils, services};
use sqlx::Postgres;

#[tokio::main]
async fn main() {
    // Init tracing/logging
    services::tracing::init_tracing();

    // Init the DB pool
    let db_pool: sqlx::Pool<Postgres> =
        services::db::init_db_conn_pool()
            .await
            .unwrap_or_else(|error| {
                panic!("Fatal problem initializng the database: {error}");
            });

    // Init the db schema and seed with sample data
    dev_and_test_utils::database_init_and_seed(db_pool.clone())
        .await
        .unwrap_or_else(|error| {
            panic!("Fatal problem initializng the database: {error}");
        });

    // Init the search service and seed the index with data
    dev_and_test_utils::search_service_init_and_seed(db_pool.clone())
        .await
        .unwrap_or_else(|error| {
            panic!("Fatal problem initializng the search service: {error}");
        });

    tracing::debug!(
        "{}",
        "Core services successfully configured and ready!"
            .green()
            .underline()
    );
}
