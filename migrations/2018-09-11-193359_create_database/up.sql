-- Your SQL goes here

CREATE TABLE shops (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL
);

CREATE TABLE products (
    id INTEGER PRIMARY KEY NOT NULL,
    shop_id INTEGER NOT NULL,
    name VARCHAR NOT NULL,
    value REAL NOT NULL,
    FOREIGN KEY(shop_id) REFERENCES shops(id)
);

CREATE TABLE orders (
    id INTEGER PRIMARY KEY NOT NULL,
    shop_id INTEGER NOT NULL,
    FOREIGN KEY(shop_id) REFERENCES shops(id)
);

CREATE TABLE line_items (
    id INTEGER PRIMARY KEY NOT NULL,
    product_id INTEGER NOT NULL,
    order_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY(product_id) REFERENCES products(id),
    FOREIGN KEY(order_id) REFERENCES orders(id)
);
