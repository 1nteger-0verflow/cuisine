CREATE TABLE IF NOT EXISTS recipes (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    name_french          TEXT NOT NULL,
    description_japanese TEXT,
    difficulty           TEXT CHECK (difficulty IN ('easy', 'medium', 'hard')),
    created_at           TEXT NOT NULL DEFAULT (datetime('now'))
);
