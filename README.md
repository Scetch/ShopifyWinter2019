# Winter 2019 Developer Intern Challenge Question

## Tools
[Rust](https://www.rust-lang.org) - Programming language
[actix_web](https://crates.io/crates/actix-web) - Web framework
[diesel](https://crates.io/crates/diesel) - ORM and Query Builder
[juniper](https://crates.io/crates/juniper) - GraphQL server library

## Setup
Install Rust
Install sqlite3

Install `diesel_cli`

```
cargo install diesel_cli --no-default-features --features sqlite3
```

```
DATABASE_URL=database.db
```

```
diesel_cli 
```

The `/graphql` endpoint will accept and respond to POST requests.

The interactive endpoint `/graphiql` is available and will provide documentation to the GraphQL schema.


## Docker

