CREATE TABLE todos (
    id           UUID        PRIMARY KEY,
    owner_id     UUID        NOT NULL,            -- foreign key added in Chapter 6
    title        TEXT        NOT NULL CHECK (char_length(title) BETWEEN 1 AND 120),
    created_at   TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ NULL
                 CHECK (completed_at IS NULL OR completed_at >= created_at)
);

CREATE INDEX todos_owner_created_idx ON todos (owner_id, created_at DESC);
