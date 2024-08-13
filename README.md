# react_app_hello_world

This demonstrates how to use Rust as the single server and runtime needed to serve up a React application written in TypeScript.

Meaning, it does not use another server for serving the React app (such as Node.js), rather Rust serves up the statically generated React application files itself. We still use node to compile the React app and generate the static files. 

We're using the Axum web application server framework for Rust.

We're using Postgres DB with the Rust SQLX client.

You should inspect the .env file at project root to verify or change to match your local configuration.