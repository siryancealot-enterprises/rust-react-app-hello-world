use dotenv::dotenv;
mod api;
mod services;

#[tokio::main]
async fn main() {
    // Load environment variables from .env (at project root... for now)
    dotenv().ok();

    // Init tracing/logging
    services::tracing::init_tracing();

    // Init the DB
    services::db::init_db_conn_pool()
        .await
        .unwrap_or_else(|error| {
            panic!("Fatal problem initializng the database: {error}");
        });

    // Init the app server
    services::app_server::init_app_server()
        .await
        .unwrap_or_else(|error| {
            panic!("Fatal problem initializng the app server: {error}");
        });
}
