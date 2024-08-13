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

pub async fn get_players() -> Json<Vec<Player>> {

    let users = sqlx::query_as!( 
        Player,
        "select number, name, email, username from player"
    )
    .fetch_all(db::utils::get_pool()) 
    .await;

    Json(users.unwrap())  
}


pub async fn get_player(Path(id): Path<i32>) -> Json<Player> {

    let users = sqlx::query_as!( 
        Player,
        "select number, name, email, username from player where number = $1",
        id
    )
    .fetch_one(db::utils::get_pool()) 
    .await;

    Json(users.unwrap())  
}

pub async fn add_player(Json(player_to_add): Json<Player>) -> Result<Json<Player>, impl IntoResponse> {

    let new_player = sqlx::query_as!(
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
    .await;

    if new_player.is_err() {
        Err((
             StatusCode::INTERNAL_SERVER_ERROR,
             format!("Failed to delete user: {}", new_player.err().expect("couldnt get error message")),
        ))
    } else {
        Ok(Json(new_player.unwrap()))

        // TODO SWY: If I wanted to send back a custom reposnse, or say use the StatusCode::CREATED response code, 
        // I'd use the code below. But it would conflict with the return type in the Error branch as the Body type 
        // would be different ). Hmmm...
        // Response::builder()
        // .status(StatusCode::CREATED)
        // .header(header::CONTENT_TYPE, "application/json")
        // .body(Json(new_player.unwrap()))
        // .unwrap()
    }
}

// END: Player API