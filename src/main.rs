use dotenv::dotenv;
use sqlx::Postgres;

use ::rust_react_app_hello_world::services;

#[tokio::main]
async fn main() {
    // Load environment variables from .env (at project root... for now)
    dotenv().ok();

    // Init tracing/logging
    services::tracing::init_tracing();

    // Init the DB
    // For now passing the connpool around, and specifically using as shared State in our app server below, which seems
    // to be the common pattern. Considered/considering using as a global shared constant with OnceCell as the backing impl.
    let db_pool: sqlx::Pool<Postgres> =
        services::db::init_db_conn_pool()
            .await
            .unwrap_or_else(|error| {
                panic!("Fatal problem initializng the database: {error}");
            });

    // Init the app server
    services::app_server::init_app_server(db_pool)
        .await
        .unwrap_or_else(|error| {
            panic!("Fatal problem initializng the app server: {error}");
        });
}
