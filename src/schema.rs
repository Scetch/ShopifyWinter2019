use std::sync::Arc;

use juniper::{ self, FieldResult, EmptyMutation };
use diesel::prelude::*;

use db::{ shops, products, orders, line_items };

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

pub struct Context {
    pub db: Arc<SqliteConnection>,
}

impl juniper::Context for Context {}

/// The Query root.
pub struct Query;

graphql_object!(Query: Context |&self| {
    description: "Query information about shops, their products, and their current orders."

    field shop(&executor, id: i32) -> FieldResult<Option<Shop>> as
        "Query a specific Shop by its id"
    {
        shops::table
            .find(id)
            .first::<Shop>(executor.context().db.as_ref())
            .optional()
            .map_err(Into::into)
    }

    field shops(&executor) -> FieldResult<Vec<Shop>> as
        "Retrieve all of the current Shops"
    {
        shops::table
            .load::<Shop>(executor.context().db.as_ref())
            .map_err(Into::into)
    }
});

/// A `Shop` contains a collection of `Product` and a collection
/// of `Order`
#[derive(Identifiable, Queryable)]
#[table_name = "shops"]
pub struct Shop {
    /// The unique ID of the `Shop`
    id: i32,
    /// The name of the `Shop`
    name: String,
}

graphql_object!(Shop: Context |&self| { 
    description: "A Shop"

    field id() -> i32 as
        "The shop id"
    {
        self.id
    }

    field name() -> &str as
        "The shop name" 
    {
        self.name.as_str()
    }

    field products(&executor) -> FieldResult<Vec<Product>> as 
        "Products for the shop"
    {
        Product::belonging_to(self)
            .load::<Product>(executor.context().db.as_ref())
            .map_err(Into::into)
    }

    field orders(&executor) -> FieldResult<Vec<Order>> as
        "Orders"
    {
        Order::belonging_to(self)
            .load::<Order>(executor.context().db.as_ref())
            .map_err(Into::into)
    }
});

/// A `Product` is a collection of items in a `Shop` that can 
/// be added to an order
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Shop, foreign_key = "shop_id")]
#[table_name = "products"]
pub struct Product {
    /// The unique ID of the `Product` 
    id: i32,
    /// The `Shop` ID the `Product` is connected to
    shop_id: i32,
    /// The name of the `Product`
    name: String,
    /// The value (price) of the `Product`
    value: f32,
}

graphql_object!(Product: Context |&self| {
    description: "A product"

    field id() -> i32 as
        "The product id"
    {
        self.id
    }

    field shop_id() -> i32 as
        "The shop id"
    {
        self.shop_id
    }

    field name() -> &str as
        "The product name"
    {
        self.name.as_str()
    }

    field value() -> f64 as
        "The product value"
    {
        self.value as f64
    }

    field line_items(&executor) -> FieldResult<Vec<LineItem>> as
        "The line items attached to this product"
    {
        LineItem::belonging_to(self)
            .load::<LineItem>(executor.context().db.as_ref())
            .map_err(Into::into)
    }
});

/// An `Order` is a collection of `LineItem` in a `Shop`
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Shop, foreign_key = "shop_id")]
#[table_name = "orders"]
pub struct Order {
    /// The unique ID of the `Order`
    id: i32,
    /// The `Shop` ID the `Order` is connceted to
    shop_id: i32,
}

graphql_object!(Order: Context |&self| {
    description: "An order"

    field id() -> i32 as
        "The order id"
    {
        self.id
    }

    field shop_id() -> i32 as
        "The shop id"
    {
        self.shop_id
    }

    field total(&executor) -> FieldResult<f64> as
        "The total amount of this order"
    {
        LineItem::belonging_to(self)
            .inner_join(products::table)
            .select((line_items::quantity, products::value))
            .load::<(i32, f32)>(executor.context().db.as_ref())
            .map(|v| {
                v.into_iter()
                    .fold(0.0, |total, (quantity, value)| total + (value * quantity as f32)) as f64
            })
            .map_err(Into::into)
    }

    field line_items(&executor) -> FieldResult<Vec<LineItem>> as
        "The line items attached to this order."
    {
        LineItem::belonging_to(self)
            .load::<LineItem>(executor.context().db.as_ref())
            .map_err(Into::into)
    }
});

/// A `LineItem` is a quantity of `Product` in an `Order`
#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Product, foreign_key = "product_id")]
#[belongs_to(Order, foreign_key = "order_id")]
#[table_name = "line_items"]
pub struct LineItem {
    /// The unique ID of the `LineItem`
    id: i32,
    /// The `Product` ID the `LineItem` is connected to
    product_id: i32,
    /// The `Order` ID the `LineItem` is connected to
    order_id: i32,
    /// The quantity of `Product` the `LineItem` contains
    quantity: i32,
}

graphql_object!(LineItem: Context |&self| {
    description: "A line item"

    field id() -> i32 as
        "The line item id."
    {
        self.id
    }

    field product_id() -> i32 as
        "The product id this line item is attached to."
    {
        self.product_id    
    }

    field order_id() -> i32 as
        "The order id this line item is attached to."
    {
        self.order_id  
    }

    field quantity() -> i32 as
        "The quantity attached to the order"    
    {
        self.quantity
    }

    field value(&executor) -> FieldResult<f64> as
        "The value of this line item. It is the price of the product multiplied by the quantity of the product."
    {  
        products::table
            .find(self.product_id)
            .select(products::value)
            .first::<f32>(executor.context().db.as_ref())
            .map(|v| (v * self.quantity as f32) as f64)
            .map_err(Into::into)
    }

    field product(&executor) -> FieldResult<Product> as
        "The product this line item is attached to"
    {
        products::table
            .find(self.product_id)
            .first::<Product>(executor.context().db.as_ref())
            .map_err(Into::into)
    }

    field order(&executor) -> FieldResult<Order> as 
        "The order this line item is attached to."
    {
        orders::table
            .find(self.order_id)
            .first::<Order>(executor.context().db.as_ref())
            .map_err(Into::into)
    }
});