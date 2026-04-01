ALTER TABLE daily_stats_background_jobs
    DROP CONSTRAINT IF EXISTS fk_user;

ALTER TABLE daily_stats_background_jobs
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;