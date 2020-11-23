-- Your SQL goes here
CREATE TABLE clients (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR NOT NULL,
  email VARCHAR,
  phone_number VARCHAR,
  address VARCHAR
);