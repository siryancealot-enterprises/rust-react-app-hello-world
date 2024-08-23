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

    use sqlx::PgPool;

    use crate::DB_MIGRATOR;

    use super::*;

    // This needs to align the number of rows inserted in the migrations script whose name contains "seeding_player_data"
    const NUM_SEED_PLAYER_ROWS: usize = 6;

    #[test]
    fn endpoints_build_id_path() {
        let result = build_id_path(PLAYERS_API);
        assert_eq!(result, format!("{}{}", PLAYERS_API, ID_PATH));
    }

    #[sqlx::test(migrator = "DB_MIGRATOR")]
    fn endpoints_get_players(pool: PgPool) {
        test_get_players(pool, NUM_SEED_PLAYER_ROWS, "kobe").await;
    }

    #[sqlx::test(migrator = "DB_MIGRATOR")]
    fn endpoints_add_player(pool: PgPool) {
        let new_player = Player {
            id: None,
            number: 31,
            username: String::from("rambo"),
            email: Some(String::from("kurt@lakers.com")),
            name: String::from("Kurt Rambis"),
        };
        let player_to_compare: Player = new_player.clone();

        let cloned_pool: PgPool = pool.clone();

        let resp: axum::http::Response<axum::body::Body> =
            add_player(axum::extract::State(pool), axum::Json(new_player))
                .await
                .into_response();

        assert_eq!(StatusCode::CREATED, resp.status());
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();

        let returned_player: Player = serde_json::from_str(
            String::from_utf8(bytes.into_iter().collect())
                .unwrap()
                .as_str(),
        )
        .unwrap();

        assert!(returned_player.id.is_some());

        // Validate an id was generated and that's a valid UUID of the format we expect...
        let id = uuid::Uuid::parse_str(returned_player.id.unwrap().to_string().as_str()).unwrap();
        assert_eq!(Some(uuid::Version::Random), id.get_version());

        assert_eq!(returned_player.username, player_to_compare.username);
        assert_eq!(returned_player.number, player_to_compare.number);
        assert_eq!(returned_player.email, player_to_compare.email);

        test_get_players(cloned_pool, NUM_SEED_PLAYER_ROWS + 1, "rambo").await;
    }

    async fn test_get_players(pool: PgPool, expected_rows: usize, username_to_validate: &str) {
        let resp = get_players(axum::extract::State(pool))
            .await
            .into_response();
        assert_eq!(StatusCode::OK, resp.status());
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_string = Json(String::from_utf8(bytes.into_iter().collect()).unwrap());

        // Parse the string of data into serde_json::Value.
        let json_parsed: serde_json::Value =
            serde_json::from_str(&body_string).expect("JSON was not well-formatted");
        assert_eq!(json_parsed.as_array().unwrap().len(), expected_rows);

        // Validate at least one player's data is returned
        assert!(body_string.contains(username_to_validate));
    }
}
