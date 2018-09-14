# Winter 2019 Developer Intern Challenge Question

Solution for the Winter 2019 Developer Intern Challenge Question.

Currently deployed on GKE (Google Kubernetes Engine) at [http://shopifyw19.scetch.net/](http://shopifyw19.scetch.net/)

The `/graphql` endpoint will accept and respond to POST requests.

The interactive endpoint `/graphiql` is available and provides documentation to the GraphQL schema.

## Tools
* [Rust](https://www.rust-lang.org) - Programming language
* [actix_web](https://crates.io/crates/actix-web) - Web framework
* [diesel](https://crates.io/crates/diesel) - ORM and Query Builder
* [juniper](https://crates.io/crates/juniper) - GraphQL server library

## Build Requirements
* Rust
* SQLite3
* OpenSSL

```
cargo run --release
```

Alternatively you can build and run with Docker.
