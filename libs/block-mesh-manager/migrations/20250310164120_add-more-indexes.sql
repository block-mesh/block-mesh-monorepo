-- Add migration script here
CREATE INDEX idx_aggregates_name_updated_user ON aggregates (name, updated_at, user_id);
CREATE INDEX idx_daily_stats_user_day_status_uptime ON daily_stats (user_id, day, status, uptime);
