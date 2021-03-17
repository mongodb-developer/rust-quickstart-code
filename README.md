# Sample Code for the MongoDB Rust Quick Start Series

This repository contains sample code for a series of blog posts describing how to use MongoDB from Rust programs.

Currently there's are two blog posts describing:

* How to get up-and-running, and how to perform some core CRUD operations: [Up and Running with Rust and MongoDB](https://developer.mongodb.com/quickstart/rust-crud-tutorial),
* How to use the aggregation framework to group and transform a collection [Getting Started with Aggregation Pipelines in Rust](https://developer.mongodb.com/quickstart/rust-quickstart-aggregation)

The blog post describes how to use the library with [Tokio](https://docs.rs/tokio/0.2.21/tokio/),
[async-std](https://docs.rs/async-std/1.6.2/async_std/),
and without an async framework.
This repo contains example code for working with Tokio because the differences are very small between the different frameworks, and this is expected to be the most common environment that people require.

If you have questions or feedback,
please let us know at the [MongoDB Community Forums](https://community.mongodb.com/)!