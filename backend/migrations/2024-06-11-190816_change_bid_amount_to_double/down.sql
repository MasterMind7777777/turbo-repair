-- This file should undo anything in `up.sql`
ALTER TABLE bids ALTER COLUMN bid_amount TYPE NUMERIC;
