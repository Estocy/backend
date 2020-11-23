-- Your SQL goes here
CREATE TABLE categories (
  id UUID PRIMARY KEY DEFAULT UUID_GENERATE_V4(),
  label VARCHAR NOT NULL,
  tag_color VARCHAR NOT NULL,
);