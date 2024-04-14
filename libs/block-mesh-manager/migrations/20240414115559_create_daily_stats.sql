CREATE TABLE daily_stats
(
    id          uuid PRIMARY KEY,
    sid         BIGSERIAL   NOT NULL UNIQUE,
    user_id     uuid        NOT NULL,
    tasks_count int         NOT NULL,
    status      text        NOT NULL,
    day         DATE        NOT NULL,
    created_at  timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)

);

CREATE INDEX daily_stats_created_at ON daily_stats (created_at);
CREATE INDEX daily_stats_status ON daily_stats (status);
CREATE INDEX daily_stats_user_id ON daily_stats (user_id);
CREATE INDEX daily_stats_day ON daily_stats (day);

CREATE TRIGGER trg_make_archive_of_changes_for_daily_stats
    AFTER INSERT OR DELETE OR UPDATE
    ON daily_stats
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('DailyStats');

