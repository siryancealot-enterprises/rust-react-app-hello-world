//! Contains common helpful testing utilities.
//! Follows the Rust convention that this file needs to be in a sub-folder and named "mod.rs":
//! https://doc.rust-lang.org/book/ch11-03-test-organization.html#submodules-in-integration-tests

use axum_test::TestServer;
use rust_react_app_hello_world::services::{self, search};

pub fn get_test_server_with_app(pool: sqlx::PgPool) -> axum_test::TestServer {
    let router: axum::Router =
        services::app_server::init_router(pool, search::get_client().unwrap());
    TestServer::new(router).unwrap()
}
