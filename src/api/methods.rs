// the beauty of this is ::sqlx will analzye at build time if your sql statements below match up with 
// your database schema. Every time you add new SQL queries, you need to run "cargo sqlx prepare" which will
// run the analysis and then report in "problems/compile errors" in your IDE that yo nee

use axum::{extract, Json};
use serde::{Serialize, Deserialize};
use crate::db;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    number: i32,
    name: String,
    username: String,
    email: Option<String>
}

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


pub const ADD_PLAYER_API: &str  = "/api/add_player";
pub async fn add_player(extract::Json(player_to_add): extract::Json<Player>) -> Json<Player> {

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
        print!("Error inserting new player: {0}", player_to_add.name);
    }
    
    Json(new_player.unwrap())
}

