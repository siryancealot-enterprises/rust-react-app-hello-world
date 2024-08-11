// the beauty of this is ::sqlx will analzye at build time if your sql statements below match up with 
// your database schema. Every time you add new SQL queries, you need to run "cargo sqlx prepare" which will
// run the analysis and then report in "problems/compile errors" in your IDE that yo nee

use axum::Json;
use serde::{Serialize, Deserialize};
use crate::db;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    number: i32,
    name: String,
    username: String,
    email: Option<String>
}

pub async fn get_players() -> Json<Vec<Player>> {

    let users = sqlx::query_as!( 
        Player,
        "select number, name, email, username from player"
    )
    .fetch_all(db::utils::get_pool()) 
    .await;

    Json(users.unwrap())  
}