# rust-react-app-hello-world

This demonstrates how to use Rust as the single server and runtime needed to serve up a React Single Page Application (SPA) written in TypeScript backed by a Postgres DB and [meilisearch](https://www.meilisearch.com/) search service. 

The SPA talks to APIs on the Rust server, which talks to the Postgres DB or Search service. 

We do not use another server for serving the React app (such as Node.js), rather the single Rust server serves up the statically generated React application files to the requesting client. We still use Node to compile the React SPA and generate its static files. 

We're using Rust's [Axum](https://docs.rs/axum/latest/axum/) web application server framework.

We're using Postgres DB with Rust's [sqlx](https://docs.rs/sqlx/latest/sqlx/) toolkit for client interaction and other DB utilities.

We're using meilisearch's [Rust SDK](https://github.com/meilisearch/meilisearch-rust).

We've added Rust unit and integration tests for this code. 

This repo relies on Docker Desktop and Docker Compose (see compose.yaml in the project's base directory) to easily automate the initalization and setup of the database and search services the application relies on.  


## How to build 

### Requirements
The instructions below assume you have the following installed locally: 
1. Git (v2.39.3+)
2. Rust (v1.80+) 
3. Node (v20.16.0+)
5. Docker Desktop (v4.32+)
6. (optional, but highly recommended) VSCode (v1.90+ with "rust-analyzer" and "ESLint" extensions installed)

Note: Postgres and meilisearch will automationcally be installed and initialized by our automated local build process using Docker Compose. 

See the [Dev Environment Setup](https://docs.google.com/document/d/1XNp3Rvjv013czinhsvlJ8TVzIqlS23XuRvU68ydVp28/edit#heading=h.nmiyh3e307cw) section for instructions on how to install any of the above.

### Instructions
With those requirements fulfilled, see the [Sync the Repo and Build the App](https://docs.google.com/document/d/1XNp3Rvjv013czinhsvlJ8TVzIqlS23XuRvU68ydVp28/edit#heading=h.xjl3tbax05i7) section on how to build this project and run the app.
