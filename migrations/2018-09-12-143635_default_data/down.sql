-- This file should undo anything in `up.sql`

DELETE FROM shops WHERE id = 1;
DELETE FROM products WHERE shop_id = 1;
DELETE FROM orders WHERE shop_id = 1;
DELETE FROM line_items WHERE id = 1;