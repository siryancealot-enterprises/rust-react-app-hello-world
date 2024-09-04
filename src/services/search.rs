use axum::extract;
use meilisearch_sdk::{client::Client, errors::Error};
use sqlx::Postgres;

use super::configs;

pub fn get_client() -> Result<Client, Error> {
    // Create a client
    Client::new(
        configs::get_env_var_or_panic("SEARCH_SERVER_URL"),
        // TODO SWY: Change to use a less priviledged "admin-focused" key
        Some(configs::get_env_var_or_panic("SEARCH_MASTER_KEY")),
    )
}

pub async fn execute_search(extract::State(pool): extract::State<sqlx::Pool<Postgres>>) {}

pub fn get_test_search_client() -> Client {
    // TODO SWY: Need to replace this with a truly mocked out impl, as this is creating a normal meilisearch Client...
    // that has to have some cost, right?
    get_client().unwrap()
}
