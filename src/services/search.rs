//! Provides utilites to interact with our Search service, at the moment: [meilisearch](https://www.meilisearch.com/).
//!
//! NOTE: At the moment we're not adding to our index automatically, it contains only what's been seeded at repo
//! initialization time. See the function in init_dev_repo.rs that does this. See [this post](https://www.meilisearch.com/docs/guides/database/meilisync_postgresql)
//! about using meilisearch to index data in Postgres.
use meilisearch_sdk::{client::Client, errors::Error};

use crate::resources::Player;

use super::configs;

pub const PLAYER_SEARCH_INDEX: &str = "players";

/// Get a Client to our Search server.
///
/// Right now, this runs with the Master key, but eventually we should make clients that only run with
/// either the "Admin API key" or the general "Search API key":
/// <https://www.meilisearch.com/docs/learn/security/differences_master_api_keys>
pub fn get_client() -> Result<Client, Error> {
    Client::new(
        configs::get_env_var_or_panic("SEARCH_SERVER_URL"),
        Some(configs::get_env_var_or_panic("SEARCH_MASTER_KEY")),
    )
}

/// Search for player(s) that match the term.  
pub async fn player_search(search_client: Client, term: &str) -> Vec<Player> {
    let search_results = search_client
        .index(PLAYER_SEARCH_INDEX)
        .search()
        .with_query(term)
        .execute::<Player>()
        .await
        .unwrap()
        .hits;

    let mut players: Vec<Player> = Vec::with_capacity(search_results.len());
    for player in search_results {
        players.push(player.result)
    }

    players
}

// TODO SWY:
// 1) Move this into test only code
// 2) Replace this with a truly mocked out impl, as this is creating a normal meilisearch Client... that has to
//    have some cost, right?
pub fn get_test_search_client() -> Client {
    get_client().unwrap()
}
