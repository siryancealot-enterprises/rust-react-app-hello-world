//! # rust-react-app-hello-world
//!
//! This demonstrates how to use Rust as the single server and runtime needed to serve up a React Single Page Application (SPA) written in TypeScript backed by a Postgres DB and [meilisearch](https://www.meilisearch.com/) search service.
//!
//! The SPA talks to APIs on the Rust server, which talks to the Postgres DB or Search service.
//!
//! We do not use another server for serving the React app (such as Node.js), rather the single Rust server serves up the statically generated React application files to the requesting client. We still use Node to compile the React SPA and generate its static files.
//!
//! ### Key technologies and libraries used:
//! * Rust's [Axum](https://docs.rs/axum/latest/axum/) web application server framework
//! * Postgres DB with Rust's [sqlx](https://docs.rs/sqlx/latest/sqlx/) toolkit for client interaction and other DB utilities
//! * Meilisearch's with its [Rust SDK](https://github.com/meilisearch/meilisearch-rust)
//! * Docker [Desktop](https://www.docker.com/products/docker-desktop/) and [Compose](https://docs.docker.com/compose/)
//!
//! ### Some best practices implemented:
//! * Added Rust unit and integration tests for most Rust code
//! * A basic GitHub workflow defined to start our core services, run tests, run Rust's formatter (rustfmt), and run Rust's linter (Clippy)
//! * The most strict linting and formatting enabled for Rust and TypeScript code configured in VSCode and GitHub Action workflow
//! * Basic logging and tracing enabled
//! * Proper environment variable support (with .toml config files in the .cargo directory)
//! * Automated DB schema creation and upgrade script execution
//! * Using Docker Desktop and Docker Compose (using the compose.yaml in the project's base directory) to easily automate the initalization and setup of the database and search services the application relies on. You can still roll your own Postgres and meilisearch local setup, but why?
//! * And many more (i.e. db connection pooling, etc. etc.)
//!
//! ## How to build

//! ### Requirements
//! The instructions below assume you have the following installed locally:
//! 1. Git (v2.39.3+)
//! 2. Rust (v1.80+)
//! 3. Node (v20.16.0+)
//! 5. Docker Desktop (v4.32+)
//! 6. (optional, but highly recommended) VSCode (v1.90+ with "rust-analyzer" and "ESLint" extensions installed)
//!
//! Note: Postgres and meilisearch will automatically be installed and initialized by our automated local build process using Docker Compose.
//!
//! See the [Dev Environment Setup](https://docs.google.com/document/d/1XNp3Rvjv013czinhsvlJ8TVzIqlS23XuRvU68ydVp28/edit#heading=h.nmiyh3e307cw) section for instructions on how to install any of the above.
//!
//! ### Instructions
//! With those requirements fulfilled, see the [Sync the Repo and Build the App](https://docs.google.com/document/d/1XNp3Rvjv013czinhsvlJ8TVzIqlS23XuRvU68ydVp28/edit#heading=h.xjl3tbax05i7) section on how to build this project and run the app.

pub mod api;
pub mod environment_utils;
pub mod services;

// Re-export our API modules for deverloper convenience and documentation
pub use api::endpoints;
pub use api::resources;

/// SQLX utility to run database creation and upgrade scripts from the migraiton sql directory
pub static DB_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
