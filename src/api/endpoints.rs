//! Exposes the public API endpoints that our service offers. The endpoints use REST naming conventions and
//! HTTP request methods to communicate intent.

/// One cool thing about Rust and the ::sqlx library is at build time it validates your sql statements in this file
/// match up with your database schema. Every time you add new SQL queries, you need to run "cargo sqlx prepare" which will
/// run the analysis and make static files avaialble in the .sqlx directory that enables offline (i.e. no database avaialble)
/// schema validation.
use crate::api::resources::Player;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::{Pool, Postgres};

// General API constants and utilities
const ID_PATH: &str = "/:id";

/// Returns a properly formatted path for retrieving a resource by id
pub fn build_id_path(resource_base_path: &str) -> String {
    format!("{}{}", resource_base_path, ID_PATH)
}

// BEGIN: Players API
pub const PLAYERS_API: &str = "/api/players";

pub async fn get_players(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let players = match sqlx::query_as!(
        Player,
        "select id, number, name, email, username from player"
    )
    .fetch_all(&pool)
    .await
    {
        Ok(players) => players,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response()
        }
    };

    (StatusCode::OK, Json(players)).into_response()
}

pub async fn get_player(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let player: Player = match sqlx::query_as!(
        Player,
        "select id, number, name, email, username from player where id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(player) => player,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response()
        }
    };

    (StatusCode::OK, Json(player)).into_response()
}

pub async fn add_player(
    State(pool): State<Pool<Postgres>>,
    Json(player_to_add): Json<Player>,
) -> impl IntoResponse {
    let new_player: Player = match sqlx::query_as!(
        Player,
        r#"INSERT INTO player
        (number, name, username, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id, number, name, username, email"#,
        player_to_add.number,
        player_to_add.name,
        player_to_add.username,
        player_to_add.email
    )
    .fetch_one(&pool)
    .await
    {
        Ok(new_player) => new_player,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response()
        }
    };

    (StatusCode::CREATED, Json(new_player)).into_response()
}

// END: Players API

#[cfg(test)]
mod tests {

    use super::*;
    use crate::DB_MIGRATOR;
    use pretty_assertions::assert_eq;
    use sqlx::PgPool;

    // This needs to align the number of rows inserted in the migrations script whose name contains "seeding_player_data"
    const NUM_SEED_PLAYER_ROWS: usize = 6;
    // The name of a Player that's inserted in our original db setup .sql script in the migrations directory
    const SEED_PLAYER_USER_NAME: &str = "kobe";

    #[test]
    fn endpoints_build_id_path() {
        let result = build_id_path(PLAYERS_API);
        assert_eq!(result, format!("{}{}", PLAYERS_API, ID_PATH));
    }

    /// Basic validaiton of our endpoint for getting a list of players
    #[sqlx::test(migrator = "DB_MIGRATOR")]
    async fn endpoints_get_players(pool: PgPool) {
        let resp = get_players(axum::extract::State(pool))
            .await
            .into_response();
        assert_eq!(StatusCode::OK, resp.status());

        let players: Vec<Player> = deserialize_api_resource(resp).await;

        assert_eq!(players.len(), NUM_SEED_PLAYER_ROWS);

        // Find an expected player, for a simple validation of the data retruned
        let mut found_the_player = false;
        for player in players {
            if player.username == SEED_PLAYER_USER_NAME {
                found_the_player = true;
                break;
            };
        }
        assert!(found_the_player);
    }

    /// Basic validaiton of our endpoint for getting a player by id
    #[sqlx::test(migrator = "DB_MIGRATOR")]
    async fn endpoints_get_player(pool: PgPool) {
        let cloned_pool = pool.clone();

        // Get the list of players and pick one to retrive by id
        let mut resp = get_players(axum::extract::State(pool))
            .await
            .into_response();
        assert_eq!(StatusCode::OK, resp.status());
        let players: Vec<Player> = deserialize_api_resource(resp).await;
        let player_to_lookup: &Player = players.get(1).unwrap();

        // Now query for the player by id
        let player_id: uuid::Uuid = player_to_lookup.id.unwrap();
        resp = get_player(
            axum::extract::State(cloned_pool),
            axum::extract::Path(player_id),
        )
        .await
        .into_response();
        assert_eq!(StatusCode::OK, resp.status());

        let returned_player: Player = deserialize_api_resource(resp).await;
        validate_players_are_same(&returned_player, player_to_lookup);
    }

    /// Basic validaiton of our endpoint for adding a new players
    #[sqlx::test(migrator = "DB_MIGRATOR")]
    async fn endpoints_add_player(pool: PgPool) {
        let new_player = Player {
            id: None,
            number: 31,
            username: String::from("rambo"),
            email: Some(String::from("kurt@lakers.com")),
            name: String::from("Kurt Rambis"),
        };
        let player_to_compare: Player = new_player.clone();

        let resp: axum::http::Response<axum::body::Body> =
            add_player(axum::extract::State(pool), axum::Json(new_player))
                .await
                .into_response();

        assert_eq!(StatusCode::CREATED, resp.status());

        let returned_player: Player = deserialize_api_resource(resp).await;
        validate_players_are_same(&returned_player, &player_to_compare);
    }

    /// Validates insert fails when missing required field
    #[sqlx::test(migrator = "DB_MIGRATOR")]
    async fn endpoints_add_player_missing_data(pool: PgPool) {
        let player_to_add = Player {
            id: None,
            number: 31,
            username: String::from("rambo"),
            email: Some(String::from("kurt@lakers.com")),
            name: String::from("Kurt Rambis"),
        };
        match sqlx::query_as!(
            Player,
            r#"INSERT INTO player
            (number, name, email)
            VALUES ($1, $2, $3)
            RETURNING id, number, name, username, email"#,
            player_to_add.number,
            player_to_add.name,
            player_to_add.email
        )
        .fetch_one(&pool)
        .await
        {
            Ok(_) => panic!("Insert improperly succeeded"),
            Err(err) => {
                assert_eq!(err.to_string(), "error returned from database: null value in column \"username\" of relation \"player\" violates not-null constraint");
            }
        };
    }

    fn validate_players_are_same(player1: &Player, player2: &Player) {
        // Validate an id was generated and that's a valid UUID of the format we expect...
        assert!(player1.id.is_some());
        let id = player1.id.unwrap();
        assert_eq!(Some(uuid::Version::Random), id.get_version());

        assert_eq!(player1.username, player2.username);
        assert_eq!(player1.number, player2.number);
        assert_eq!(player1.email, player2.email);
        assert_eq!(player1.name, player2.name);
    }

    async fn deserialize_api_resource<T: serde::de::DeserializeOwned>(
        resp: axum::http::Response<axum::body::Body>,
    ) -> T {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();

        serde_json::from_slice(&bytes).unwrap()
    }
}
