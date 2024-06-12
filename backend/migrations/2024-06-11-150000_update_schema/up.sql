-- Create repair_shops table
CREATE TABLE repair_shops (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create addresses table
CREATE TABLE addresses (
    id UUID PRIMARY KEY,
    repair_shop_id UUID REFERENCES repair_shops(id),
    street VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    state VARCHAR NOT NULL,
    zip VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create staff table
CREATE TABLE staff (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    repair_shop_id UUID REFERENCES repair_shops(id),
    role VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create customers table (users table already exists)
-- No changes needed for customers, they are part of users table

-- Create repair_requests table
CREATE TABLE repair_requests (
    id UUID PRIMARY KEY,
    customer_id UUID REFERENCES users(id),
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create bids table
CREATE TABLE bids (
    id UUID PRIMARY KEY,
    repair_request_id UUID REFERENCES repair_requests(id),
    repair_shop_id UUID REFERENCES repair_shops(id),
    bid_amount DECIMAL NOT NULL,
    status VARCHAR NOT NULL, -- pending, accepted, rejected
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    repair_request_id UUID REFERENCES repair_requests(id),
    repair_shop_id UUID REFERENCES repair_shops(id),
    status VARCHAR NOT NULL, -- ongoing, completed, transferred
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create status_pipeline table
CREATE TABLE status_pipeline (
    id UUID PRIMARY KEY,
    order_id UUID REFERENCES orders(id),
    status VARCHAR NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW()
);

