-- Your SQL goes here
CREATE TABLE products_categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  product_id UUID references products(id) ON DELETE CASCADE,
  category_id UUID references categories(id) ON DELETE CASCADE,
  label VARCHAR NOT NULL,
  tag_color VARCHAR NOT NULL
);