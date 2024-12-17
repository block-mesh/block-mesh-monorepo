CREATE TABLE daily_stats_background_jobs
(
    id         UUID        NOT NULL DEFAULT gen_random_uuid(),
    day        DATE        NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX daily_stats_background_job_id ON daily_stats_background_jobs (id);
CREATE INDEX daily_stats_background_job_day ON daily_stats_background_jobs (day);