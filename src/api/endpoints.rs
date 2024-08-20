//! Exposes the public API endpoints that our service offers. The endpoints use REST naming conventions and
//! HTTP request methods.

/// One cool thing about Rust and the ::sqlx library is at build time it validates your sql statements in this file
/// match up with your database schema. Every time you add new SQL queries, you need to run "cargo sqlx prepare" which will
/// run the analysis and make static files avaialble in the .sqlx directory that enables offline (i.e. no database avaialble)
/// schema validation.
use crate::api::entities::Player;
use crate::services::db;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::Router, Json};

// BEGIN: Players API
const PLAYERS_API: &str = "/api/players";

async fn get_players() -> impl IntoResponse {
    let players = match sqlx::query_as!(
        Player,
        "select id, number, name, email, username from player"
    )
    .fetch_all(db::get_pool())
    .await
    {
        Ok(players) => players,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response()
        }
    };

    (StatusCode::OK, Json(players)).into_response()
}

async fn get_player(Path(id): Path<uuid::Uuid>) -> impl IntoResponse {
    let player: Player = match sqlx::query_as!(
        Player,
        "select id, number, name, email, username from player where id = $1",
        id
    )
    .fetch_one(db::get_pool())
    .await
    {
        Ok(player) => player,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response()
        }
    };

    (StatusCode::OK, Json(player)).into_response()
}

async fn add_player(Json(player_to_add): Json<Player>) -> impl IntoResponse {
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
    .fetch_one(db::get_pool())
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

/// Add all API endpoints to our App Server's router. This should only be used during App Server initalization.
pub fn add_all_endpoints(router: Router) -> Router {
    router
        // Add all Player API endpoints
        .route(PLAYERS_API, axum::routing::get(get_players))
        .route(
            format!("{}{}", PLAYERS_API, "/:id").as_str(),
            axum::routing::get(get_player),
        )
        .route(PLAYERS_API, axum::routing::put(add_player))
}