-- Your SQL goes here
CREATE TYPE scoring_type AS ENUM('MP', 'IMP');

CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT,
    date DATE NOT NULL,
    owner_id UUID NOT NULL,
    scoring_type "scoring_type" NOT NULL,
    should_use_victory_points BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (owner_id) REFERENCES users(id)
);