CREATE TABLE IF NOT EXISTS terms (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    french     TEXT NOT NULL,
    japanese   TEXT NOT NULL,
    category   TEXT NOT NULL CHECK (category IN ('dish', 'ingredient', 'utensil', 'technique')),
    notes      TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_terms_category ON terms(category);
CREATE INDEX idx_terms_french   ON terms(french);
