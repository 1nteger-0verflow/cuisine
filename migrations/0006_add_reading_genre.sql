ALTER TABLE terms ADD COLUMN reading TEXT;
ALTER TABLE terms ADD COLUMN genre TEXT CHECK (genre IN (
  'soup', 'stew', 'dessert', 'pastry', 'main', 'appetizer',
  'dairy', 'herb', 'spice', 'vegetable', 'mushroom', 'protein', 'grain', 'seafood'
));
