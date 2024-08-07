# react_app_hello_world

This demonstrates how to use Rust as the single server needed to serve up a React application written in TypeScript.

Meaning, it does not use another server for the React app (such as Node.js), rather Rust serves up the statically generated React application files itself.

We're using the Axum web application server framework for Rust.