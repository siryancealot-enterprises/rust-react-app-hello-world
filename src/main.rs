use dotenv::dotenv;
mod api;
mod services;

#[tokio::main]
async fn main() {
    // Load environment variables from .env (at project root... for now)
    dotenv().ok();

    if services::db::init_db_conn_pool().await.is_err() {
        panic!("PANIC: DB UNHEALTHY, check logs")
    }

    if services::app_server::init_app_server().await.is_err() {
        panic!("PANIC: APP SERVER UNHEALTHY, check logs")
    }
}
