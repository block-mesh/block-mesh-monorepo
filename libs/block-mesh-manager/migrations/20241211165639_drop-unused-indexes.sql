DROP INDEX new_daily_stats_status;
DROP INDEX daily_stats_ref_bonus;
DROP INDEX daily_stats_ref_bonus_applied;
DROP INDEX new_aggregates_status;
CREATE INDEX daily_stats_finalized_status ON daily_stats_finalized (status);
CREATE INDEX daily_stats_default_status ON daily_stats_default (status);
