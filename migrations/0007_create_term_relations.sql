CREATE TABLE IF NOT EXISTS term_relations (
  from_id       INTEGER NOT NULL REFERENCES terms(id) ON DELETE CASCADE,
  to_id         INTEGER NOT NULL REFERENCES terms(id) ON DELETE CASCADE,
  relation_type TEXT    NOT NULL DEFAULT 'related',
  PRIMARY KEY (from_id, to_id),
  CHECK (from_id != to_id)
);

CREATE INDEX idx_term_relations_to ON term_relations(to_id);
