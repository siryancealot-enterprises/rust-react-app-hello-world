
use dotenv::dotenv;

mod db;
mod api;
mod app_server;


#[tokio::main]
async fn main() {

    // Load environment variables from .env (at project root... for now)
    dotenv().ok(); 

    if db::utils::init_db_conn_pool().await.is_err() {
        panic!("PANIC: DB UNHEALTHY, check logs")
    }
    
   if app_server::utils::init_app_server().await.is_err() {
        panic!("PANIC: APP SERVER UNHEALTHY, check logs")
    }

}
