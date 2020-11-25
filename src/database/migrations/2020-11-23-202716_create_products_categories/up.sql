-- Your SQL goes here
CREATE TABLE products_categories (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  product_id UUID references products(id) ON DELETE CASCADE NOT NULL,
  category_id UUID references categories(id) ON DELETE CASCADE NOT NULL
);