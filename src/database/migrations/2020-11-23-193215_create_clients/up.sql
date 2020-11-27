-- Your SQL goes here
CREATE TABLE clients (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID references users(id) ON DELETE CASCADE NOT NULL,
  name VARCHAR NOT NULL,
  email VARCHAR,
  phone_number VARCHAR,
  address VARCHAR
);