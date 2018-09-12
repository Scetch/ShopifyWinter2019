table! {
    line_items (id) {
        id -> Integer,
        product_id -> Integer,
        value -> Float,
    }
}

table! {
    orders (id) {
        id -> Integer,
        shop_id -> Integer,
    }
}

table! {
    products (id) {
        id -> Integer,
        shop_id -> Integer,
        name -> Text,
        value -> Float,
    }
}

table! {
    shops (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(line_items -> products (product_id));
joinable!(orders -> shops (shop_id));
joinable!(products -> shops (shop_id));

allow_tables_to_appear_in_same_query!(
    line_items,
    orders,
    products,
    shops,
);
