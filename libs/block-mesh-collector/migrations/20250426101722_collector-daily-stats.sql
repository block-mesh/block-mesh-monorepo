CREATE TABLE collector_daily_stats
(
    id         uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    day        DATE        NOT NULL,
    count      int         NOT NULL
);

CREATE INDEX collector_daily_stats_created_at ON collector_daily_stats (created_at);
CREATE UNIQUE INDEX collector_daily_stats_day ON collector_daily_stats (day);
CREATE INDEX collector_daily_stats_count ON collector_daily_stats (count);
