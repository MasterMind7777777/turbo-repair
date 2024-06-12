-- Your SQL goes here
ALTER TABLE bids
    ALTER COLUMN repair_request_id SET NOT NULL,
    ALTER COLUMN repair_shop_id SET NOT NULL;
