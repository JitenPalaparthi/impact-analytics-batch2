-- init.sql
CREATE TABLE IF NOT EXISTS users (
    id          BIGSERIAL PRIMARY KEY,
    name        TEXT        NOT NULL,
    email       TEXT        NOT NULL UNIQUE,
    age         INT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);