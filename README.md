# rust-react-app-hello-world

This demonstrates how to use Rust as the single server and runtime needed to serve up a React Single Page Application (SPA) written in TypeScript. 

The SPA talks to APIs on the Rust server, which talks to a Postgres DB. 

We do not use another server for serving the React app (such as Node.js), rather the single Rust server serves up the statically generated React application files to the requesting client. We still use Node to compile the React SPA and generate its static files. 

We're using Rust's [Axum](https://docs.rs/axum/latest/axum/) web application server framework.

We're using Postgres DB with Rust's [sqlx](https://docs.rs/sqlx/latest/sqlx/) toolkit for client interaction and other DB utilities.


## How to build 

### Requirements
The instructions below assume you have the following installed locally: 
1. Git 
2. Rust (v1.80+) 
3. Node (v20.16.0+)
4. Postgres (v16.4+) running and available (via Docker or otherwise)
5. (optional) VSCode (v1.90+ with "rust-analyzer" and "ESLint" extensions installed) 

See the [Dev Environment Setup](https://docs.google.com/document/d/1Xh-SnX5DuJubVoiXvrFGsymZAgfo_j1fL0gJeouUldA/edit#heading=h.nmiyh3e307cw) section for instructions on how to install any of the above.

### Instructions
With those requirements fulfilled, see the [Sync the Repo and Build the App](https://docs.google.com/document/d/1Xh-SnX5DuJubVoiXvrFGsymZAgfo_j1fL0gJeouUldA/edit#heading=h.xjl3tbax05i7) section on how to build this project and run the app.
