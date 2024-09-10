//! Provides utilites to interact with our Search service, at the moment: [meilisearch](https://www.meilisearch.com/).
//!
//! NOTE: At the moment we're adding to our Player index at repo initialization time, see [`environment_utils::dev_and_test_utils`](`environment_utils::dev_and_test_utils`), and
//! at player creation time, see: [`api::endpoints::add_player`](`create::api::endpoints::add_player`). Eventually we may want to have an automated system to is based on our
//! DB replication/notification type approach.  See [this post](https://www.meilisearch.com/docs/guides/database/meilisync_postgresql)
//! about using meilisearch to index data in Postgres.
//!
//! TODO SWY: Tests from other modules currently use the same index as the normal running application locally, this needs to be separated
//! via some test specific .env value for the index name for tests, creating a search service mock, or some other approach.
use std::time;

use colored::Colorize;
use meilisearch_sdk::{client::Client, errors::Error, task_info::TaskInfo, tasks::Task};

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
/// /// Note: this is broken out from the search function above for testing purposes.
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

/// Upserts a Player to the default Players search index
pub async fn index_player(search_client: &Client, player: &Player) {
    let result = index_player_with_idx(search_client, player, PLAYER_SEARCH_INDEX).await;
    if result.is_err() {
        // For production, we'll need to send some alert or feed some operator system to monitor and bulk fix
        tracing::error!("{} {:?}", "Serch indexing error".red(), result.err());
    }
}

/// Upserts a Player to the search index specified
/// Note: this is broken out from the index function above for testing purposes.
async fn index_player_with_idx(
    search_client: &Client,
    player: &Player,
    index: &str,
) -> Result<TaskInfo, Error> {
    let mut players: Vec<&Player> = Vec::with_capacity(1);
    players.push(player);
    search_client
        .index(index)
        .add_documents(&players, Some("id"))
        .await
}

/// Convienence utility to wait for an indexing operation to complete. This function hardcodes checking
/// every 100 millis and will timeout after 30 seconds. If that doesn't suffice, make a new parameterized function!
pub async fn wait_for_search_operation_to_complete(
    search_client: &Client,
    task_info: TaskInfo,
) -> Result<Task, Error> {
    search_client
        .wait_for_task(
            task_info,
            Some(time::Duration::from_millis(100)),
            Some(time::Duration::from_secs(30)),
        )
        .await
}

#[cfg(test)]
pub(crate) mod search_test_utils {
    use meilisearch_sdk::client::Client;

    use super::get_client;

    pub fn get_test_search_client() -> Client {
        // TODO SWY: Replace this with a truly mocked out impl, or setup a client with a Test specific index
        get_client().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{environment_utils::dev_and_test_utils, resources::Player, DB_MIGRATOR};
    use dotenv::dotenv;
    use meilisearch_sdk::client::Client;
    use sqlx::PgPool;
    use uuid::Uuid;

    /// Since we need to create unique indexes per test (to avoid stomping within and between test runs),
    /// generate a name that combines the base of the index name plus the current time in millis.
    fn get_unique_test_index_name(idx_name: &str) -> String {
        let mut test_index_name: String = idx_name.to_owned();
        let milliseconds_timestamp: u128 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        test_index_name.push_str(u128::to_string(&milliseconds_timestamp).as_str());
        test_index_name
    }

    #[sqlx::test(migrator = "DB_MIGRATOR")]
    async fn search_player_search(pool: PgPool) {
        let test_index_name: String = get_unique_test_index_name("search_player_search");
        // Initialize and seed the search index (using our test's name for the index)
        let search_client: Client =
            dev_and_test_utils::search_service_init_and_seed_with_idx(pool, &test_index_name)
                .await
                .unwrap();

        let players: Vec<Player> =
            player_search_with_idx(&search_client, "kobe", &test_index_name).await;
        assert_eq!(players.len(), 1);
        assert_eq!(players.first().unwrap().username, "kobe");

        // delete the test-specific index
        search_client.delete_index(&test_index_name).await.unwrap();
    }

    #[tokio::test]
    async fn search_index_player() {
        // For some reason we need to manually load .env file (though it seems intermittent, should investigate further)
        dotenv().ok();

        let test_index_name = get_unique_test_index_name("search_index_player");

        // Initialize the search index (using our test's name for the index)
        let search_client: Client = dev_and_test_utils::search_service_init(&test_index_name)
            .await
            .unwrap();

        let username: &str = "rambo";
        let player = Player {
            id: Some(Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").unwrap()),
            number: 31,
            username: String::from(username),
            email: Some(String::from("kurt@lakers.com")),
            name: String::from("Kurt Rambis"),
        };

        let insert_task: TaskInfo =
            index_player_with_idx(&search_client, &player, &test_index_name)
                .await
                .unwrap();

        // Wait for indexing to finish...
        wait_for_search_operation_to_complete(&search_client, insert_task)
            .await
            .unwrap();

        let players: Vec<Player> =
            player_search_with_idx(&search_client, username, &test_index_name).await;
        assert_eq!(players.len(), 1);
        assert_eq!(players.first().unwrap().username, username);

        // delete the test-specific index
        search_client.delete_index(&test_index_name).await.unwrap();
    }
}
