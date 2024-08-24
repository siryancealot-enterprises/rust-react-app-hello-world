//! API integration tests
//! Note, currently these tests are essentially duplicate tests of the unit tests in ['api::endpoints']. But they're here
//! to show:
//!     1) how you'd test with our app server
//!     2) verifies the app server's API Routes and MethodHandlers are setup properly that simulates  external calls to our APIs (maybe there's a better way to do this?)
use react_app_hello_world::api::{endpoints, resources::Player};

mod test_utils;

/// Basic validaiton of our endpoint for getting a list of players
#[sqlx::test(migrator = "react_app_hello_world::DB_MIGRATOR")]
fn api_get_players(pool: sqlx::PgPool) {
    let server = test_utils::get_test_server_with_app(pool);

    let response = server.get(endpoints::PLAYERS_API).await;
    assert_eq!(response.status_code(), axum::http::StatusCode::OK);

    let players: Vec<Player> = response.json::<Vec<Player>>();
    assert_eq!(players.len(), 6);
}

/// Basic validaiton of our endpoint for getting a player by id
#[sqlx::test(migrator = "react_app_hello_world::DB_MIGRATOR")]
fn api_get_player(pool: sqlx::PgPool) {
    let server = test_utils::get_test_server_with_app(pool);

    // Get the list of all players and select one at random...
    let mut response = server.get(endpoints::PLAYERS_API).await;
    assert_eq!(response.status_code(), axum::http::StatusCode::OK);

    let players: Vec<Player> = response.json::<Vec<Player>>();
    let player: &Player = players.get(1).unwrap();

    // Now query back for the player by id
    let player_id: uuid::Uuid = player.id.unwrap();

    response = server
        .get(format!("{}/{}", endpoints::PLAYERS_API, player_id).as_str())
        .await;

    assert_eq!(response.status_code(), axum::http::StatusCode::OK);

    let retrieved_player: Player = response.json::<Player>();
    assert_eq!(retrieved_player.id, player.id);

    validate_players_match(player, &retrieved_player);
}

/// Basic validaiton of our endpoint for adding a new players
#[sqlx::test(migrator = "react_app_hello_world::DB_MIGRATOR")]
fn api_add_player(pool: sqlx::PgPool) {
    let server = test_utils::get_test_server_with_app(pool);

    let player_to_create = Player {
        id: None,
        number: 31,
        username: String::from("rambo"),
        email: Some(String::from("kurt@lakers.com")),
        name: String::from("Kurt Rambis"),
    };

    let req: axum_test::TestRequest = server.put(endpoints::PLAYERS_API).json(&player_to_create);
    let response = req.await;

    assert_eq!(response.status_code(), axum::http::StatusCode::CREATED);

    let returned_player: Player = response.json::<Player>();
    assert!(returned_player.id.is_some());
    assert_eq!(
        Some(uuid::Version::Random),
        returned_player.id.unwrap().get_version()
    );
    validate_players_match(&player_to_create, &returned_player);
}

fn validate_players_match(player1: &Player, player2: &Player) {
    assert_eq!(player1.username, player2.username);
    assert_eq!(player1.number, player2.number);
    assert_eq!(player1.email, player2.email);
    assert_eq!(player1.name, player2.name);
}
