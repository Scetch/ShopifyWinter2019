extern crate actix_web;
extern crate diesel;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

use actix_web::{ App, Responder, HttpRequest, Json, Path, State, };

/*
GET /shop/
POST /shop/

GET /shop/{id}
PUT /shop/{id}
DELETE /shop/{id}
*/

#[derive(Debug, Serialize, Deserialize)]
struct Shop {
    products: Vec<Product>,
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    line_items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Order {}

struct AppState {}

fn main() {
    actix_web::server::new(|| {
            App::with_state(AppState {})
                .resource("/", |r| r.with(index))
                .scope("/shop/", |s| {
                    s.resource("", |r| {
                            r.get().f(|_| "Get");
                            r.post().f(|_| "Post");
                        })
                        .resource("/{id}/", |r| {
                            r.get().f(|_| "Get");
                            r.put().f(|_| "Put");
                            r.delete().f(|_| "Delete");
                        })
                })
                .default_resource(|r| r.f(|_| {
                    Json(json!({
                        "error": "Resource not found."
                    }))
                }))
        })
        .bind("127.0.0.1:80")
        .unwrap()
        .run();
}

fn index(_: State<AppState>) -> impl Responder {
    Json(vec![Shop {
        products: vec![
            Product {
                line_items: vec![
                    "Apple".into(),
                ],
            },
        ],
        orders: vec![

        ],
    }])
}

fn shop_create(_: State<AppState>) -> impl Responder {
    ""
}

fn shop_read(_: State<AppState>) -> impl Responder {
    "BLAH"
}

fn shop_update(_: State<AppState>) -> impl Responder {
    ""
}

fn shop_delete(_: State<AppState>) -> impl Responder {
    ""
}
