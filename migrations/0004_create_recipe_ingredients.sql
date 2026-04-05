CREATE TABLE IF NOT EXISTS recipe_ingredients (
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    term_id   INTEGER NOT NULL REFERENCES terms(id),
    quantity  TEXT,
    notes     TEXT,
    PRIMARY KEY (recipe_id, term_id)
);
