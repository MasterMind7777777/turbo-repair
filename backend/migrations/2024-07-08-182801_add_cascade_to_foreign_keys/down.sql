ALTER TABLE addresses
DROP CONSTRAINT addresses_repair_shop_id_fkey,
ADD CONSTRAINT addresses_repair_shop_id_fkey
FOREIGN KEY (repair_shop_id)
REFERENCES repair_shops(id);

ALTER TABLE bids
DROP CONSTRAINT bids_repair_shop_id_fkey,
ADD CONSTRAINT bids_repair_shop_id_fkey
FOREIGN KEY (repair_shop_id)
REFERENCES repair_shops(id);

ALTER TABLE orders
DROP CONSTRAINT orders_repair_shop_id_fkey,
ADD CONSTRAINT orders_repair_shop_id_fkey
FOREIGN KEY (repair_shop_id)
REFERENCES repair_shops(id);

ALTER TABLE staff
DROP CONSTRAINT staff_repair_shop_id_fkey,
ADD CONSTRAINT staff_repair_shop_id_fkey
FOREIGN KEY (repair_shop_id)
REFERENCES repair_shops(id);

