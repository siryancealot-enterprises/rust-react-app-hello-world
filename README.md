# rust-react-app-hello-world

This demonstrates how to use Rust as the single server and runtime needed to serve up a React application written in TypeScript.

Meaning, it does not use another server for serving the React app (such as Node.js), rather Rust serves up the statically generated React application files itself. We still use Node to compile the React app and generate the static files. 

We're using the Axum web application server framework for Rust.

We're using Postgres DB with the Rust SQLX client.

You should inspect the .env file at project root to verify or change to match your local configuration.

## How to build 

### Requirements
This assumes you have the following already installed locally: VSCode, Git, Rust, Node, and a working Postgres database (in Docker or otherwise). Here are instructions on how to install any of these: [https://docs.google.com/document/d/1Xh-SnX5DuJubVoiXvrFGsymZAgfo_j1fL0gJeouUldA/edit
](https://docs.google.com/document/d/1Xh-SnX5DuJubVoiXvrFGsymZAgfo_j1fL0gJeouUldA/edit#heading=h.nmiyh3e307cw)

### Instructions
With those requirements fulfilled, here's how to build this project: [https://docs.google.com/document/d/1Xh-SnX5DuJubVoiXvrFGsymZAgfo_j1fL0gJeouUldA/edit#heading=h.xjl3tbax05i7](https://docs.google.com/document/d/1Xh-SnX5DuJubVoiXvrFGsymZAgfo_j1fL0gJeouUldA/edit#heading=h.xjl3tbax05i7)
