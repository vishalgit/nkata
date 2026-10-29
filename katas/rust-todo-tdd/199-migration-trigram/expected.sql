CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX todos_title_trgm_idx ON todos USING gin (title gin_trgm_ops);
