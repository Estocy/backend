-- Your SQL goes here
CREATE TABLE categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  label VARCHAR NOT NULL,
  tag_color VARCHAR NOT NULL
);