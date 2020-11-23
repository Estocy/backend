-- Your SQL goes here
CREATE TABLE products (
  id UUID PRIMARY KEY DEFAULT UUID_GENERATE_V4(),
  name VARCHAR PRIMARY KEY,
  description VARCHAR NOT NULL,
  store_name VARCHAR,
  store_price DECIMAL,
  price DECIMAL NOT NULL,
  additional_charge DECIMAL DEFAULT 0.00,
  color VARCHAR NOT NULL,
  weight DECIMAL NOT NULL,
  brand VARCHAR NOT NULL,
  stock_amount INTEGER NOT NULL,
);