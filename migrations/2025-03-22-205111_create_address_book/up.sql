-- Your SQL goes here
CREATE TABLE contacts(
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    phone_number TEXT,
    email TEXT,
    UNIQUE (first_name, last_name)
);