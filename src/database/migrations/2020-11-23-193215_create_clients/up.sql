-- Your SQL goes here
CREATE TABLE clients (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  phone_number VARCHAR NOT NULL,
  address VARCHAR NOT NULL
);