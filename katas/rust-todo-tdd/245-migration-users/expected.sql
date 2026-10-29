CREATE TABLE users (
    id            UUID        PRIMARY KEY,
    email         TEXT        NOT NULL UNIQUE,
    password_hash TEXT        NOT NULL,
    role          TEXT        NOT NULL DEFAULT 'user' CHECK (role IN ('user', 'admin')),
    created_at    TIMESTAMPTZ NOT NULL
);

-- Rows from Chapter 5 have no real owner. Remove them, then enforce ownership.
DELETE FROM todos WHERE owner_id NOT IN (SELECT id FROM users);
ALTER TABLE todos
    ADD CONSTRAINT todos_owner_fk FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE;
