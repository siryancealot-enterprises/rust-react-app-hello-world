pub mod api;
pub mod services;

/// SQLX utility to run database creation and upgrade scripts from the migraiton sql directory
pub static DB_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
