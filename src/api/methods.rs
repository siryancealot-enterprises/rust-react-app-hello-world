// the beauty of this is ::sqlx will analzye at build time if your sql statements below match up with 
// your database schema. Every time you add new SQL queries, you need to run "cargo sqlx prepare" which will
// run the analysis and then report in "problems/compile errors" in your IDE that yo nee

use axum::{ extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde::{ Deserialize, Serialize};
use crate::db;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    number: i32,
    name: String,
    username: String,
    email: Option<String>
}

// BEGIN: Player API
pub const PLAYERS_API: &str  = "/api/players";

pub async fn get_players() -> impl IntoResponse {

    let players = match sqlx::query_as!( 
        Player,
        "select number, name, email, username from player"
    )
    .fetch_all(db::utils::get_pool()) 
    .await
    {
        Ok(players) => players,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err.to_string()),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(players)).into_response()
}


pub async fn get_player(Path(id): Path<i32>) -> impl IntoResponse {

    let player:Player = match sqlx::query_as!( 
        Player,
        "select number, name, email, username from player where number = $1",
        id
    )
    .fetch_one(db::utils::get_pool()) 
    .await
    {
        Ok(player) => player,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err.to_string()),
            )
                .into_response()
        }
    };

    (StatusCode::OK, Json(player)).into_response()
}

pub async fn add_player(Json(player_to_add): Json<Player>) -> impl IntoResponse {

    let new_player:Player = match sqlx::query_as!(
        Player,
        r#"INSERT INTO player
        (number, name, username, email)
        VALUES ($1, $2, $3, $4)
        RETURNING number, name, username, email"#,
        player_to_add.number,
        player_to_add.name,
        player_to_add.username,
        player_to_add.email
    )
    .fetch_one(db::utils::get_pool()) 
    .await
    {
        Ok(new_player) => new_player,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err.to_string()),
            )
                .into_response()
        }
    };

    (StatusCode::CREATED, Json(new_player)).into_response()
}

// END: Player API