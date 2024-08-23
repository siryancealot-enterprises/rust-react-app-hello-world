pub mod api;
pub mod services;

pub static DB_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
