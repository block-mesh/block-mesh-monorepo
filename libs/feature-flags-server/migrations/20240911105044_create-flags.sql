CREATE TABLE flags
(
    id        uuid PRIMARY KEY,
    name      TEXT        NOT NULL,
    value     JSONB       NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX flags_created_at ON flags (created_at);
CREATE UNIQUE INDEX flags_name ON flags (name);