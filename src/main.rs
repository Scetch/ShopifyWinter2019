extern crate actix_web;
#[macro_use] extern crate diesel;
extern crate env_logger;
#[macro_use] extern crate juniper;

use diesel::prelude::*;
use std::sync::Arc;

use actix_web::{ App, Json, State, Responder, HttpRequest, HttpResponse, };
use juniper::http::GraphQLRequest;

mod db;
mod schema;

pub struct AppState {
    db: Arc<SqliteConnection>,
    executor: schema::Schema,
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    actix_web::server::new(|| {
            let db = Arc::new(SqliteConnection::establish("database.db")
                .expect("Couldn't connect to Sqlite Database."));

            let executor = schema::Schema::new(schema::Query, juniper::EmptyMutation::new());

            App::with_state({
                    AppState {
                        db: db,
                        executor: executor,
                    }
                })
                .resource("/", |r| {
                    r.f(|_| {
                        HttpResponse::Found()
                            .header("location", "/graphiql")
                            .finish()
                    })
                })
                .resource("/graphql", |r| {
                    r.post().with(graphql_post);
                })
                .resource("/graphiql", |r| {
                    r.get().with(graphiql);
                })
        })
        .bind("0.0.0.0:8000")
        .unwrap()
        .run();
}

fn graphql_post((state, req): (State<AppState>, Json<GraphQLRequest>)) -> impl Responder {
    let req = req.into_inner();
    let resp = req.execute(&state.executor, &schema::Context { db: state.db.clone() });

    if resp.is_ok() {
        HttpResponse::Ok().json(resp)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

fn graphiql(_: HttpRequest<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .header("Content-Type", "text/html; charset=UTF-8")
        .body(juniper::graphiql::graphiql_source("/graphql"))
}
