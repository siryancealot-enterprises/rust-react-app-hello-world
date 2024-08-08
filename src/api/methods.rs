use axum::Json;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    username: String
}

pub async fn get_users() -> Json<Vec<User>> {
    
    // Some JSON input data as a &str. Maybe this comes from the user.
     let mut user = User {
         id: 24,
         name: "Kobe Bryant".to_owned(),
         email: "kobe@lakers.com".to_owned(),
         username: "kobe_bryant".to_owned()
     };

    let mut v: Vec<User> = Vec::new();
    v.push(user);

    user = User {
        id: 32,
        name: "Shaquile O'Neal".to_owned(),
        email: "shaq@lakers.com".to_owned(),
        username: "shaq_oneal".to_owned()
    };

    v.push(user);

    Json(v)
}