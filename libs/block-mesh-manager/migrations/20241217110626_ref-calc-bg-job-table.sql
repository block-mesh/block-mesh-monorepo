CREATE TABLE daily_stats_background_jobs
(
    id         UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    day        DATE             NOT NULL,
    user_id    uuid             NOT NULL,
    created_at timestamptz      NOT NULL DEFAULT now(),
    updated_at timestamptz      NOT NULL DEFAULT now(),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX daily_stats_background_job_user_id ON daily_stats_background_jobs (user_id);
CREATE INDEX daily_stats_background_job_day ON daily_stats_background_jobs (day);
CREATE UNIQUE INDEX daily_stats_background_job_day_user_id ON daily_stats_background_jobs (day, user_id);