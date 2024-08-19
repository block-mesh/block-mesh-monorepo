ALTER TABLE aggregates ADD COLUMN updated_at timestamptz NOT NULL DEFAULT now();

CREATE INDEX aggregates_updated_at ON aggregates (updated_at);
