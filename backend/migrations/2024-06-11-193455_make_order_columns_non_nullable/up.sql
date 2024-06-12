-- Your SQL goes here
ALTER TABLE orders
    ALTER COLUMN repair_request_id SET NOT NULL,
    ALTER COLUMN repair_shop_id SET NOT NULL;

