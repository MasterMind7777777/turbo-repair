-- This file should undo anything in `up.sql`
ALTER TABLE bids
    ALTER COLUMN repair_request_id DROP NOT NULL,
    ALTER COLUMN repair_shop_id DROP NOT NULL;
