use std::sync::Arc;

use juniper::{ self, FieldResult, EmptyMutation };
use diesel::prelude::*;

use db::{ shops, products, orders, line_items };

///
/// A Shop GraphQL object.
/// 
#[derive(GraphQLObject)]
#[graphql(description = "A shop")]
pub struct Shop {
    #[graphql(description = "The id of the shop.")]
    id: i32,
    #[graphql(description = "The name of the shop.")]
    name: String,
    #[graphql(description = "Products related to the shop.")]
    products: Vec<Product>,
    #[graphql(description = "Orders related to the shop.")]
    orders: Vec<Order>,
}

///
/// A Product GraphQL object.
/// 
#[derive(GraphQLObject)]
#[graphql(description = "A product")]
pub struct Product {
    #[graphql(description = "The id of the product.")]
    id: i32,
    #[graphql(description = "The shop_id this product is attached to.")]
    shop_id: i32,
    #[graphql(description = "The name of this product.")]
    name: String,
    #[graphql(description = "The value of this product.")]
    value: f64,
}

///
/// An Order GraphQL object.
/// 
#[derive(GraphQLObject)]
#[graphql(description = "An order")]
pub struct Order {
    #[graphql(description = "The id of the order.")]
    id: i32,
    #[graphql(description = "The shop_id this product is attached to.")]
    shop_id: i32,
}

///
/// A LineItem GraphQL Object.
/// 
#[derive(GraphQLObject)]
#[graphql(description = "A line item")]
pub struct LineItem {
    #[graphql(description = "The id of the line item.")]
    id: i32,
    #[graphql(description = "The id of the product this line item is attached to,")]
    product_id: i32,
    #[graphql(description = "The id of the order this line item is attached to.")]
    order_id: i32,
    #[graphql(description = "The quantity of product this line item holds.")]
    quantity: i32,
}

pub struct Context {
    pub db: Arc<SqliteConnection>,
}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query: Context |&self| {
    description: "Query shops, their products, and their current orders."

    field shop(&executor, id: i32) -> FieldResult<Option<Shop>> {
        let db = executor.context().db.as_ref();

        let res = shops::table.filter(shops::id.eq(id))
            .first::<(i32, String)>(db)
            .and_then(|(id, name)| {
                let products = products::table.filter(products::shop_id.eq(id))
                    .load::<(i32, i32, String, f32)>(db)?
                    .into_iter()
                    .map(|(id, shop_id, name, value)| {
                        Product {
                            id: id,
                            shop_id: shop_id,
                            name: name,
                            value: value as f64,
                        }
                    })
                    .collect();

                let orders = orders::table.filter(orders::shop_id.eq(id))
                    .load::<(i32, i32)>(db)?
                    .into_iter()
                    .map(|(id, shop_id)| {
                        Order {
                            id: id,
                            shop_id: shop_id,
                        }
                    })
                    .collect();

                Ok(Shop {
                    id: id,
                    name: name,
                    products: products,
                    orders: orders,
                })
            })
            .optional()?;

        Ok(res)
    }

    field shops(&executor) -> FieldResult<Vec<Shop>> {
        let db = executor.context().db.as_ref();

        let res = shops::table.load::<(i32, String)>(db)?
            .into_iter()
            .map(|(id, name)| {
                let products = products::table.filter(products::shop_id.eq(id))
                    .load::<(i32, i32, String, f32)>(db)?
                    .into_iter()
                    .map(|(id, shop_id, name, value)| {
                        Product {
                            id: id,
                            shop_id: shop_id,
                            name: name,
                            value: value as f64,
                        }
                    })
                    .collect();

                let orders = orders::table.filter(orders::shop_id.eq(id))
                    .load::<(i32, i32)>(db)?
                    .into_iter()
                    .map(|(id, shop_id)| {
                        Order {
                            id: id,
                            shop_id: shop_id,
                        }
                    })
                    .collect(); 

                Ok(Shop {
                    id: id,
                    name: name,
                    products: products,
                    orders: orders,
                })
            })
            .collect::<QueryResult<_>>()?;

        Ok(res)
    }
});

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;
