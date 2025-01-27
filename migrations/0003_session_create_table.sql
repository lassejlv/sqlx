CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    token TEXT NOT NULL
);

CREATE UNIQUE INDEX sessions_token_idx ON sessions (token);

CREATE INDEX sessions_user_id_idx ON sessions (user_id);
