ALTER TABLE daily_stats
    ADD COLUMN updated_at timestamptz NOT NULL DEFAULT now();