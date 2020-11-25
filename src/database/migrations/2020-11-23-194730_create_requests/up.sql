-- Your SQL goes here
CREATE TABLE requests (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID references users(id) ON DELETE CASCADE,
  client_id UUID references clients(id) ON DELETE CASCADE,
  sale_date DATE NOT NULL,
  delivery_date DATE NOT NULL,
  status INTEGER NOT NULL,
  comments VARCHAR,
  price REAL NOT NULL,
  discount REAL NOT NULL,
  freight REAL NOT NULL
);