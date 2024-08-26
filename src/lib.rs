//! # my_rust_react_app_hello_world
//!
//! `my_rust_react_app_hello_world` demonstrates how to use Rust as the single server and runtime needed to serve up a React Single Page Application (SPA) written in TypeScript backed by a Postgres DB.
//!
//! The SPA talks to APIs on the Rust server, which talks to a Postgres DB.
//!
//! We do not use another server for serving the React app (such as Node.js), rather the single Rust server serves up the statically generated React application files to the requesting client. We still use Node to compile the React SPA and generate its static files.
//!
//! We're using Rust's [Axum](https://docs.rs/axum/latest/axum/) web application server framework.
//!
//! We're using Postgres DB with Rust's [sqlx](https://docs.rs/sqlx/latest/sqlx/) toolkit for client interaction and other DB utilities.
//!
//! We've added Rust unit and integration tests for this code.
pub mod api;
pub mod services;

/// SQLX utility to run database creation and upgrade scripts from the migraiton sql directory
pub static DB_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
