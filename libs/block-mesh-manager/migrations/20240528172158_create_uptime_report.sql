CREATE TABLE uptime_reports
(
    id         uuid PRIMARY KEY,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    user_id    uuid        NOT NULL,
    nonce      uuid        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX uptime_reports_created_at ON uptime_reports (created_at);
CREATE INDEX uptime_reports_user_id ON uptime_reports (user_id);
CREATE INDEX uptime_reports_nonce ON uptime_reports (nonce);

CREATE TRIGGER trg_make_archive_of_changes_for_uptime_reports
    AFTER INSERT OR DELETE OR UPDATE
    ON invite_codes
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('UptimeReport');

