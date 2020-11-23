-- Your SQL goes here
CREATE TABLE requests_products (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  request_id UUID references requests(id) ON DELETE CASCADE,
  product_id UUID references clients(id) ON DELETE CASCADE,
  amount INTEGER NOT NULL,
  additional_costs DECIMAL NOT NULL,
  discount DECIMAL NOT NULL
);