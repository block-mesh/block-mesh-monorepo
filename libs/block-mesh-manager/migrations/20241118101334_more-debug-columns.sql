ALTER TABLE daily_stats
    ADD COLUMN ws_uptime_bonus DOUBLE PRECISION DEFAULT 0.0 NOT NULL;
ALTER TABLE daily_stats
    ADD COLUMN ws_tasks_count_bonus INT DEFAULT 0 NOT NULL;
