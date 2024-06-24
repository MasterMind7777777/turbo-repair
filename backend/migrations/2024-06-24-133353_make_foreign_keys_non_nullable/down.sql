-- This file should undo anything in `up.sql`
-- Down migration
ALTER TABLE addresses ALTER COLUMN repair_shop_id DROP NOT NULL;
ALTER TABLE staff ALTER COLUMN user_id DROP NOT NULL;
ALTER TABLE staff ALTER COLUMN repair_shop_id DROP NOT NULL;
ALTER TABLE status_pipeline ALTER COLUMN order_id DROP NOT NULL;
