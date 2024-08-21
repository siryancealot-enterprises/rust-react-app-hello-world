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
