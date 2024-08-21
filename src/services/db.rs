//! Provides utilities to initialize usage of the database and provide functions to interact with it.
use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;

use crate::services::configs;

// This allows (I believe) a singleton Connectionp Pool that can be shared for the life-time of the applicaitonn.
// We dole out the pool in get_pool() function below.
static CONN_POOL: OnceCell<Pool<Postgres>> = OnceCell::const_new();

/// Initalize a DB Conn Pool with with the following features:
///  
/// 1. Pool size of number of connections as defined by DATABASE_MAX_CONNECTIONS in .env file
/// 2. Connetion acquire timeout
/// 3. Prints out the connection string (with password redacted)
///
pub async fn init_db_conn_pool() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(configs::get_env_var_as_number_or_panic(
            "DATABASE_MAX_CONNECTIONS",
        ))
        .acquire_timeout(Duration::from_secs(u64::from(
            configs::get_env_var_as_number_or_panic("DATABASE_CONNECTION_ACQUIRE_TIMEOUT"),
        )))
        .connect(get_db_connect_string().as_str())
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    CONN_POOL.set(pool)?;

    tracing::debug!("DB ready for business: {:?}", row.0 > 0);

    Ok(())
}

// Return the DB connect string from the .env file, priting out the string with the DB password redacted.
fn get_db_connect_string() -> String {
    let password: String = configs::get_env_var_or_panic("DATABASE_PASSWORD");
    let connect_string: String = configs::get_env_var_or_panic("DATABASE_URL");
    let redacted_connect_string: String = connect_string.replace(&password, "<password_redacted>");
    tracing::debug!("DB Connect str: {0}", redacted_connect_string);

    connect_string
}

/// Returns the initailzed Database Connection pool which can serve DB connections to use.
// TODO SWY: does this just hand a DB connection as an effect of the &'static" syntax?  If so, rename.
pub fn get_pool() -> &'static Pool<Postgres> {
    return CONN_POOL
        .get()
        .expect("ALERT: Don't have an availalbe pool anymore!");
}
