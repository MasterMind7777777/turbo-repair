-- up.sql

-- Enable the uuid-ossp extension to generate UUIDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE user_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    repair_shop_id UUID NOT NULL REFERENCES repair_shops(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE staff_tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    author_id UUID NOT NULL REFERENCES staff(id),
    repair_shop_id UUID NOT NULL REFERENCES repair_shops(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE task_staff_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL REFERENCES staff_tasks(id) ON DELETE CASCADE,
    staff_id UUID NOT NULL REFERENCES staff(id),
    UNIQUE (task_id, staff_id)
);
