//! Provides utilities to initialize usage of the database and provide functions to interact with it.
use std::time::Duration;

use colored::Colorize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::services::configs;

/// Initalize a DB Conn Pool with with the following features:
///  
/// 1. Pool size of number of connections as defined by DATABASE_MAX_CONNECTIONS in .env file
/// 2. Connection acquire timeout
/// 3. Prints out the connection string (with password redacted)
///
pub async fn init_db_conn_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool: Pool<Postgres> = PgPoolOptions::new()
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

    assert!(row.0 > 0);

    tracing::debug!(
        "{}",
        "Database connection pool created and ready for requests".green()
    );

    Ok(pool)
}

// Return the DB connect string from the .env file, priting out the string
fn get_db_connect_string() -> String {
    let db_connect_string: String = configs::get_env_var_or_panic("DATABASE_URL");
    tracing::debug!("DB Connect str: {0}", db_connect_string);

    db_connect_string
}
