-- sauces テーブル作成
CREATE TABLE sauces (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  french     TEXT NOT NULL,
  reading    TEXT,
  genre      TEXT CHECK(genre IN ('mere','derivee','froide','emulsionnee','beurre')),
  notes      TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- term_relations の CHECK 制約に 'sauce' を追加（テーブル再作成）
CREATE TABLE term_relations_new (
  from_category TEXT    NOT NULL CHECK(from_category IN ('dish','ingredient','utensil','technique','sauce')),
  from_id       INTEGER NOT NULL,
  to_category   TEXT    NOT NULL CHECK(to_category   IN ('dish','ingredient','utensil','technique','sauce')),
  to_id         INTEGER NOT NULL,
  relation_type TEXT    NOT NULL DEFAULT 'related',
  PRIMARY KEY (from_category, from_id, to_category, to_id),
  CHECK (NOT (from_category = to_category AND from_id = to_id))
);
INSERT INTO term_relations_new SELECT * FROM term_relations;
DROP TABLE term_relations;
ALTER TABLE term_relations_new RENAME TO term_relations;
