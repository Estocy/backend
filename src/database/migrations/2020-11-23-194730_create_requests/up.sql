-- Your SQL goes here
CREATE TABLE requests (
  id UUID PRIMARY KEY DEFAULT UUID_GENERATE_V4(),
  user_id UUID references user(id) ON DELETE CASCADE,
  client_id UUID references client(id) ON DELETE CASCADE,
  sale_date DATE NOT NULL,
  delivery_date DATE NOT NULL,
  status INTEGER NOT NULL,
  comments VARCHAR PRIMARY KEY,
  price DECIMAL NOT NULL,
  discount DECIMAL NOT NULL,
  freight DECIMAL NOT NULL
);