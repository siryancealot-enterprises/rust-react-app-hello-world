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
pub async fn player_search(search_client: &Client, term: &str) -> Vec<Player> {
    player_search_with_idx(search_client, term, PLAYER_SEARCH_INDEX).await
}

/// Search for player(s) that match the term against a specific index (i.e not the default index).  
async fn player_search_with_idx(search_client: &Client, term: &str, index: &str) -> Vec<Player> {
    let search_results = search_client
        .index(index)
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

#[cfg(test)]
pub(crate) mod search_test_utils {
    use meilisearch_sdk::client::Client;

    use super::get_client;

    // TODO SWY: Replace this with a truly mocked out impl, or setup a client with a Test specific index
    pub fn get_test_search_client() -> Client {
        get_client().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{environment_utils::dev_and_test_utils, resources::Player, DB_MIGRATOR};
    use meilisearch_sdk::client::Client;
    use sqlx::PgPool;

    #[sqlx::test(migrator = "DB_MIGRATOR")]
    async fn endpoints_player_search(pool: PgPool) {
        let test_index_name: &str = "endpoints_player_search";
        // Initialize and seed the search index (using our test's name for the index)
        let search_client: Client =
            dev_and_test_utils::search_service_init_and_seed(pool, test_index_name)
                .await
                .unwrap();

        let players: Vec<Player> =
            player_search_with_idx(&search_client, "kobe", test_index_name).await;
        assert_eq!(players.len(), 1);
        assert_eq!(players.get(0).unwrap().username, "kobe");

        // delete the test-specific index (for the next run)
        search_client.delete_index(test_index_name).await.unwrap();
    }
}
