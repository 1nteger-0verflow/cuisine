CREATE TABLE IF NOT EXISTS recipe_steps (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id            INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    step_number          INTEGER NOT NULL,
    instruction_french   TEXT,
    instruction_japanese TEXT,
    UNIQUE (recipe_id, step_number)
);
