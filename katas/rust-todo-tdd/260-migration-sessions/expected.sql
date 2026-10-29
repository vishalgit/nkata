CREATE TABLE sessions (
    token_hash TEXT        PRIMARY KEY,           -- SHA-256 of the cookie value
    user_id    UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);
CREATE INDEX sessions_user_idx ON sessions (user_id);
