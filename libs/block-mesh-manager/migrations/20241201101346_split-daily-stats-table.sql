ALTER TABLE daily_stats
    RENAME TO daily_stats_old;
----------
CREATE TABLE daily_stats
(
    id                   uuid                         NOT NULL DEFAULT gen_random_uuid(),
    user_id              uuid                         NOT NULL,
    tasks_count          INT              DEFAULT 0   NOT NULL,
    tasks_count_bonus    INT              DEFAULT 0   NOT NULL,
    status               text                         NOT NULL,
    day                  DATE                         NOT NULL,
    created_at           timestamptz                  NOT NULL DEFAULT now(),
    updated_at           timestamptz                  NOT NULL DEFAULT now(),
    uptime               DOUBLE PRECISION DEFAULT 0.0 NOT NULL,
    uptime_bonus         DOUBLE PRECISION DEFAULT 0.0 NOT NULL,
    ws_uptime_bonus      DOUBLE PRECISION DEFAULT 0.0 NOT NULL,
    ws_tasks_count_bonus INT              DEFAULT 0   NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (status, id)
) PARTITION BY LIST (status);
----------
CREATE INDEX new_daily_stats_id ON daily_stats (id);
CREATE INDEX new_daily_stats_user_id ON daily_stats (user_id);
CREATE INDEX new_daily_stats_status ON daily_stats (status);
CREATE INDEX new_daily_stats_day ON daily_stats (day);
CREATE UNIQUE INDEX new_daily_stats_status_user_id_name ON daily_stats (status, day, user_id);
----------
CREATE TABLE daily_stats_finalized PARTITION OF daily_stats FOR VALUES IN ('Finalized');
CREATE TABLE daily_stats_on_going PARTITION OF daily_stats FOR VALUES IN ('OnGoing');
---------
CREATE TABLE daily_stats_default PARTITION OF daily_stats DEFAULT;
---------
INSERT INTO daily_stats (id, user_id, tasks_count, tasks_count_bonus, status, day, created_at, updated_at, uptime,
                         uptime_bonus, ws_uptime_bonus, ws_tasks_count_bonus)
SELECT id,
       user_id,
       tasks_count,
       tasks_count_bonus,
       status,
       day,
       created_at,
       updated_at,
       uptime,
       uptime_bonus,
       ws_uptime_bonus,
       ws_tasks_count_bonus
FROM daily_stats_old;
---------
DROP TABLE daily_stats_old;

