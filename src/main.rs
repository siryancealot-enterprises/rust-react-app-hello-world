
use core::panic;
use axum::{extract::Query, response::Html};
use serde::Deserialize;
use rand::{thread_rng, Rng};
use dotenv::dotenv;

mod db;
mod api;
mod app_server;
 

#[tokio::main]
async fn main()  {

    // Load environment variables from .env (at project root... for now)
    dotenv().ok(); 


    if db::utils::init_db_conn_pool().await.is_err() {
        panic!("PANIC: DB UNHEALTHY, check logs")
    }

    if app_server::utils::init_app_server().await.is_err() {
        panic!("PANIC: APP SERVER UNHEALTHY, check logs")
    }

}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn random_number_handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1></br><a href='/'>Go Back</a>", random_number))
}