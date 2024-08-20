use dotenv::dotenv;
mod api;
mod services;

#[tokio::main]
async fn main() {
    // Load environment variables from .env (at project root... for now)
    dotenv().ok();

    services::db::init_db_conn_pool().await;

    services::app_server::init_app_server().await;
}
