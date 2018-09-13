-- Your SQL goes here

INSERT INTO shops(name) VALUES ("My Shop");
INSERT INTO products(shop_id, name, value) VALUES (1, "Apple", 5.0);
INSERT INTO orders(shop_id) VALUES (1);
INSERT INTO line_items(product_id, order_id, quantity) VALUES (1, 1, 2);