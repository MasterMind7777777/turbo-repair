-- Your SQL goes here
-- Up migration
ALTER TABLE addresses ALTER COLUMN repair_shop_id SET NOT NULL;
ALTER TABLE staff ALTER COLUMN user_id SET NOT NULL;
ALTER TABLE staff ALTER COLUMN repair_shop_id SET NOT NULL;
ALTER TABLE status_pipeline ALTER COLUMN order_id SET NOT NULL;
