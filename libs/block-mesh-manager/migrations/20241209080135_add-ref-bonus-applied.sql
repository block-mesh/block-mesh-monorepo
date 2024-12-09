ALTER TABLE daily_stats
    ADD COLUMN ref_bonus DOUBLE PRECISION DEFAULT 0.0 NOT NULL;
ALTER TABLE daily_stats
    ADD COLUMN ref_bonus_applied BOOLEAN DEFAULT FALSE NOT NULL;
CREATE INDEX daily_stats_ref_bonus ON daily_stats (ref_bonus);
CREATE INDEX daily_stats_ref_bonus_applied ON daily_stats (ref_bonus_applied);
