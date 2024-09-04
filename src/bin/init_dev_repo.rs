//! A utility for initalizing repos on developer or test environments. This utility...
//!
//! 1. Creates the database's initial schema and seeds with some sample data
//! 2. Creates the search index(es) and seeds it with some sample data from the DB.
//!
//! Both utilities assume the database and search service are running, available, and configured to what's in .env.
//!
//! As the application grows in scope and schema, you'll need to update this utility accordingly.
//!
//! TODO SWY: A production-ized version of this is needed that only focuses on db schema and index creation, with
//! whatever legitimate/real data seeding needed.
//!
use std::{thread, time};

use axum::response::IntoResponse;
use dotenv::dotenv;
use meilisearch_sdk::{client::*, errors::Error, indexes::Index, task_info::TaskInfo, tasks};
use rust_react_app_hello_world::{
    endpoints,
    resources::Player,
    services::{self, app_server::AppState, configs, search},
    DB_MIGRATOR,
};
use sqlx::{migrate::MigrateError, Postgres};

#[tokio::main]
async fn main() {
    // Load environment variables from .env (at project root... for now)
    dotenv().ok();

    // Init tracing/logging
    services::tracing::init_tracing();

    // Init the DB
    let db_pool: sqlx::Pool<Postgres> =
        services::db::init_db_conn_pool()
            .await
            .unwrap_or_else(|error| {
                panic!("Fatal problem initializng the database: {error}");
            });

    database_init_and_seed(db_pool.clone())
        .await
        .unwrap_or_else(|error| {
            panic!("Fatal problem initializng the database: {error}");
        });

    search_service_init_and_seed(db_pool)
        .await
        .unwrap_or_else(|error| {
            panic!("Fatal problem initializng the search service: {error}");
        });
}

/// Runs the sqlx::migrate::Migrator, which creates the database's initial schema and seeds with some sample data
async fn database_init_and_seed(db_pool: sqlx::Pool<Postgres>) -> Result<(), MigrateError> {
    // let migrator = Migrator::new(path::Path::new("./migrations")).await?;
    // migrator.run(&db_pool).await
    DB_MIGRATOR.run(&db_pool).await
}

/// Creates the search indexes and seeds it with some sample data from the DB.
/// Note: when using search for real, we'll rely on a real system for notifications of data insert or change
/// from whatever the source system is (DB, file service, etc.)
async fn search_service_init_and_seed(db_pool: sqlx::Pool<Postgres>) -> Result<(), Error> {
    // Create a client
    let client = Client::new(
        configs::get_env_var_or_panic("SEARCH_SERVER_URL"),
        // TODO SWY: Change to use a less priviledged "admin-focused" key
        Some(configs::get_env_var_or_panic("SEARCH_MASTER_KEY")),
    )
    .unwrap();

    // The index where our Player resource data is stored.
    let players_idx = client.index(search::PLAYER_SEARCH_INDEX);

    // If we want to enable filtering, we must add the attributes to the filterableAttributes index setting.
    // You only need to perform this operation once.
    // Note that Meilisearch will rebuild your index whenever you update filterableAttributes. Depending on the size of your
    // dataset, this might take time. You can track the process using the returned Tasks.
    let filterable_attributes = ["id", "name"];
    client
        .index(search::PLAYER_SEARCH_INDEX)
        .set_filterable_attributes(&filterable_attributes)
        .await
        .expect("Failed creatinng the index filter attribute");

    // Insert the Player data into the index
    let mut insert_task: TaskInfo = seed_player_index(client.clone(), &players_idx, db_pool).await;

    // Wait for indexing to finish...
    // TODO SWY: put some type of timeout to protect infinite spining
    let mut task: tasks::Task;
    while insert_task.status != "succeeded" {
        // Give a brief pause so we're not hammering the server
        tracing::info!("Waiting for Search indexing to complete...:");
        thread::sleep(time::Duration::from_millis(250));

        task = client.get_task(insert_task.clone()).await?;
        if task.is_success() {
            insert_task.status = "succeeded".to_string();
            break;
        } else if task.is_failure() {
            panic!("error seeding serach index with data")
        }
    }

    // Run a test serach to ensure data was loaded and indexed (and hence available) ...
    let search_results = client
        .index(search::PLAYER_SEARCH_INDEX)
        .search()
        .with_query("kobe")
        .execute::<Player>()
        .await
        .unwrap()
        .hits;

    assert_eq!(search_results.len(), 1);
    println!("{:?}", search_results);

    Ok(())
}

/// Read in all Players from the DB and insert them in bulk into the Search index
async fn seed_player_index(
    search_client: Client,
    players_idx: &Index,
    db_pool: sqlx::Pool<Postgres>,
) -> TaskInfo {
    let resp = endpoints::get_players(axum::extract::State(AppState {
        db_pool,
        search_client,
    }))
    .await
    .into_response();
    let players: Vec<Player> = endpoints::deserialize_api_resource(resp).await;

    players_idx
        .add_documents(&players, Some("id"))
        .await
        .unwrap()
}