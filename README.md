# Sample Code for the MongoDB Rust Quick Start Series

This repository contains sample code for a series of blog posts describing how to use MongoDB from Rust programs.

Currently there's only one blog post describing how to get up-and-running,
and how to perform some core CRUD operations:
[Up and Running with Rust and MongoDB](https://developer.mongodb.com/quickstart/rust-crud-tutorial)

The blog post describes how to use the library with [Tokio](https://docs.rs/tokio/0.2.21/tokio/),
[async-std](https://docs.rs/async-std/1.6.2/async_std/),
and without an async framework, so there are three very similar branches,
demonstrating the different approaches:

* The [tokio](https://github.com/mongodb-developer/rust-quickstart-code/blob/tokio/src/main.rs) branch.
* The [async-std](https://github.com/mongodb-developer/rust-quickstart-code/blob/async-std/src/main.rs) branch.
* The [sync](https://github.com/mongodb-developer/rust-quickstart-code/blob/sync/src/main.rs) branch.

If you have questions or feedback,
please let us know at the [MongoDB Community Forums](https://community.mongodb.com/)!