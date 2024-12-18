CREATE TABLE log_entries
(
    id         uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    raw        TEXT        NOT NULL
);

CREATE INDEX log_entries_created_at ON log_entries (created_at);
CREATE INDEX log_entries_updated_at ON log_entries (updated_at);
