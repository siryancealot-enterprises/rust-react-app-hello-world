use meilisearch_sdk::{client::Client, errors::Error};

use crate::resources::Player;

use super::configs;

pub const PLAYER_SEARCH_INDEX: &str = "players";

pub fn get_client() -> Result<Client, Error> {
    Client::new(
        configs::get_env_var_or_panic("SEARCH_SERVER_URL"),
        // TODO SWY: Change to use a less priviledged "admin-focused" key
        Some(configs::get_env_var_or_panic("SEARCH_MASTER_KEY")),
    )
}

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

// TODO SWY: move this into test only code
pub fn get_test_search_client() -> Client {
    // TODO SWY: Need to replace this with a truly mocked out impl, as this is creating a normal meilisearch Client...
    // that has to have some cost, right?
    get_client().unwrap()
}
