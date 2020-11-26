-- Your SQL goes here
CREATE TABLE requests_products (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  request_id UUID references requests(id) ON DELETE CASCADE NOT NULL,
  product_id UUID references products(id) ON DELETE CASCADE NOT NULL,
  amount INTEGER NOT NULL,
  additional_costs REAL NOT NULL,
  discount REAL NOT NULL
);