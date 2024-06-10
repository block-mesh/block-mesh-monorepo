CREATE TABLE bandwidth_reports
(
    id              uuid PRIMARY KEY,
    sid             BIGSERIAL NOT NULL UNIQUE,
    user_id         uuid NOT NULL,
    created_at      timestamptz NOT NULL,
    download_speed  DOUBLE PRECISION NOT NULL,
    upload_speed    DOUBLE PRECISION NOT NULL,
    latency         DOUBLE PRECISION NOT NULL,
    country         TEXT NOT NULL,
    ip              TEXT NOT NULL,
    asn             TEXT NOT NULL,
    colo            TEXT NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX bandwidth_reports_created_at ON bandwidth_reports (created_at);
CREATE INDEX bandwidth_reports_download_speed ON bandwidth_reports (download_speed);
CREATE INDEX bandwidth_reports_upload_speed ON bandwidth_reports (upload_speed);
CREATE INDEX bandwidth_reports_latency ON bandwidth_reports (latency);
CREATE INDEX bandwidth_reports_country ON bandwidth_reports (country);
CREATE INDEX bandwidth_reports_ip ON bandwidth_reports (ip);
CREATE INDEX bandwidth_reports_asn ON bandwidth_reports (asn);
CREATE INDEX bandwidth_reports_colo ON bandwidth_reports (colo);

CREATE TRIGGER trg_make_archive_of_changes_for_bandwidth_reports
    AFTER INSERT OR DELETE OR UPDATE
    ON bandwidth_reports
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('BandwidthReport');