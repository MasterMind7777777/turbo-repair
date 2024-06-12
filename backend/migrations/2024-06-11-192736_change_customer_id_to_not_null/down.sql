-- This file should undo anything in `up.sql`
ALTER TABLE repair_requests ALTER COLUMN customer_id DROP NOT NULL;
