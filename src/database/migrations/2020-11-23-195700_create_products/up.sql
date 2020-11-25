-- Your SQL goes here
CREATE TABLE products (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  store_name VARCHAR,
  store_price REAL,
  price REAL NOT NULL,
  additional_charge REAL DEFAULT 0.00,
  color VARCHAR NOT NULL,
  weight REAL NOT NULL,
  brand VARCHAR NOT NULL,
  stock_amount INTEGER NOT NULL
);