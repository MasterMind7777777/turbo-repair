-- up.sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

