//! Utilities for initalizing repos on developer or test environments. This utility...
//!
//! 1. Creates the database's initial schema and seeds with some sample data
//! 2. Creates the search index(es) and seeds it with some sample data from the DB.
//!
//! Both utilities assume the database and search service are running, available, and configured to what's in .env.
//!
//! As the application grows in scope and schema, you'll need to update this utility accordingly.
//!
//! A production-ized version of this will be needed that only focuses on creating db schema and index creation, along with
//! whatever legitimate/real data seeding is needed.
//!
//! A production-ized version of this will be needed that only focuses on creating db schema and index creation, along with
//! whatever legitimate/real data seeding is needed.
use std::time;

use axum::response::IntoResponse;
use meilisearch_sdk::{client::*, errors::Error, indexes::Index, task_info::TaskInfo};
use sqlx::{migrate::MigrateError, PgPool, Postgres};

use crate::{
    endpoints,
    resources::Player,
    services::{self, app_server::AppState},
    DB_MIGRATOR,
};

// Name that should be in our seed data, which we can test query against...
const PLAYER_NAME_TO_FIND: &str = "kobe";

/// Creates the search indexes and seeds it with some sample data from the DB.
/// Note: when using search for real, we'll rely on a real system for notifications of data insert or change
/// from whatever the source system is (DB, file service, etc.)
pub async fn search_service_init_and_seed(
    db_pool: PgPool,
    index_name: &str,
) -> Result<Client, Error> {
    // Create a serach client
    let client = services::search::get_client()?;

    // The index where our Player resource data is stored.
    let players_idx = client.index(index_name);

    // If we want to enable filtering, we must add the attributes to the filterableAttributes index setting.
    // You only need to perform this operation once.
    // Note that Meilisearch will rebuild your index whenever you update filterableAttributes. Depending on the size of your
    // dataset, this might take time. You can track the process using the returned Tasks.
    let filterable_attributes = ["id", "name"];
    client
        .index(index_name)
        .set_filterable_attributes(&filterable_attributes)
        .await
        .expect("Failed creatinng the index filter attribute");

    // Insert the Player data into the index
    let insert_task: TaskInfo = seed_player_index(client.clone(), &players_idx, db_pool).await;

    // Wait for indexing to finish...
    client
        .wait_for_task(
            insert_task,
            Some(time::Duration::from_millis(250)),
            Some(time::Duration::from_secs(30)),
        )
        .await?;

    // Run a test serach to ensure data was loaded and indexed (and hence available) ...
    let search_results = client
        .index(index_name)
        .search()
        .with_query(PLAYER_NAME_TO_FIND)
        .execute::<Player>()
        .await
        .unwrap()
        .hits;

    assert!(search_results.len() > 0);
    println!("{:?}", search_results);

    Ok(client)
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

/// Runs the sqlx::migrate::Migrator, which creates the database's initial schema and seeds with some sample data
pub async fn database_init_and_seed(db_pool: sqlx::Pool<Postgres>) -> Result<(), MigrateError> {
    // let migrator = Migrator::new(path::Path::new("./migrations")).await?;
    // migrator.run(&db_pool).await
    DB_MIGRATOR.run(&db_pool).await
}
