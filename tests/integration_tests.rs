use react_app_hello_world::api::resources;
use sqlx::PgPool;

#[sqlx::test(migrator = "react_app_hello_world::DB_MIGRATOR")]
async fn sql_validate_seeded_player_data(pool: PgPool) -> sqlx::Result<()> {
    let players = match sqlx::query_as!(
        resources::Player,
        "select id, number, name, email, username from player"
    )
    .fetch_all(&pool)
    .await
    {
        Ok(players) => players,
        Err(err) => {
            panic!("error retrieving get_players {0}", err);
        }
    };

    assert_eq!(players.len(), 6);

    Ok(())
}
