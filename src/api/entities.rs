//! Defines the entities that our public APIs provide.

use serde::{Deserialize, Serialize};
use uuid::{self, Uuid};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: Option<Uuid>,
    pub number: i32,
    pub name: String,
    pub username: String,
    pub email: Option<String>,
}
