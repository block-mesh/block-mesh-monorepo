-- Add migration script here
CREATE INDEX daily_stats_updated_at on daily_stats (updated_at);
