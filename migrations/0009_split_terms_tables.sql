-- (1) 新テーブル作成
CREATE TABLE dishes (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  french     TEXT NOT NULL,
  japanese   TEXT NOT NULL,
  reading    TEXT,
  genre      TEXT CHECK(genre IN ('soup','stew','dessert','pastry','main','appetizer')),
  notes      TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE ingredients (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  french     TEXT NOT NULL,
  japanese   TEXT NOT NULL,
  reading    TEXT,
  genre      TEXT CHECK(genre IN ('dairy','herb','spice','vegetable','mushroom','protein','grain','seafood')),
  notes      TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE utensils (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  french     TEXT NOT NULL,
  japanese   TEXT NOT NULL,
  reading    TEXT,
  notes      TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE techniques (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  french     TEXT NOT NULL,
  japanese   TEXT NOT NULL,
  reading    TEXT,
  notes      TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- (2) terms からデータを移行（ID を保持）
INSERT INTO dishes (id, french, japanese, reading, genre, notes, created_at)
  SELECT id, french, japanese, reading, genre, notes, created_at FROM terms WHERE category = 'dish';

INSERT INTO ingredients (id, french, japanese, reading, genre, notes, created_at)
  SELECT id, french, japanese, reading, genre, notes, created_at FROM terms WHERE category = 'ingredient';

INSERT INTO utensils (id, french, japanese, reading, notes, created_at)
  SELECT id, french, japanese, reading, notes, created_at FROM terms WHERE category = 'utensil';

INSERT INTO techniques (id, french, japanese, reading, notes, created_at)
  SELECT id, french, japanese, reading, notes, created_at FROM terms WHERE category = 'technique';

-- (3) recipe_ingredients を ingredient_id 参照に変更
CREATE TABLE recipe_ingredients_new (
  recipe_id     INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
  ingredient_id INTEGER NOT NULL REFERENCES ingredients(id) ON DELETE CASCADE,
  quantity      TEXT,
  notes         TEXT,
  PRIMARY KEY (recipe_id, ingredient_id)
);
INSERT INTO recipe_ingredients_new (recipe_id, ingredient_id, quantity, notes)
  SELECT recipe_id, term_id, quantity, notes FROM recipe_ingredients;
DROP TABLE recipe_ingredients;
ALTER TABLE recipe_ingredients_new RENAME TO recipe_ingredients;

-- (4) term_relations をカテゴリー横断対応に変更
DROP TABLE term_relations;
CREATE TABLE term_relations (
  from_category TEXT    NOT NULL CHECK(from_category IN ('dish','ingredient','utensil','technique')),
  from_id       INTEGER NOT NULL,
  to_category   TEXT    NOT NULL CHECK(to_category   IN ('dish','ingredient','utensil','technique')),
  to_id         INTEGER NOT NULL,
  relation_type TEXT    NOT NULL DEFAULT 'related',
  PRIMARY KEY (from_category, from_id, to_category, to_id),
  CHECK (NOT (from_category = to_category AND from_id = to_id))
);

-- (5) 旧 terms テーブルを削除
DROP TABLE terms;
