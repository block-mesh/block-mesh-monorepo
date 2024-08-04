CREATE TABLE ip_addresses
(
    id              uuid PRIMARY KEY,
    sid             BIGSERIAL NOT NULL UNIQUE,
    ip              TEXT NOT NULL UNIQUE,
    created_at      timestamptz NOT NULL,
    latitude        DOUBLE PRECISION,
    longitude       DOUBLE PRECISION,
    country         TEXT,
    city            TEXT,
    region          TEXT,
    timezone        TEXT,
    isp             TEXT,
    enriched        bool NOT NULL
);


CREATE INDEX ip_addresses_reports_created_at ON ip_addresses (created_at);
CREATE INDEX ip_addresses_reports_ip ON ip_addresses (ip);
CREATE INDEX ip_addresses_reports_latitude ON ip_addresses (latitude);
CREATE INDEX ip_addresses_reports_longitude ON ip_addresses (longitude);
CREATE INDEX ip_addresses_reports_country ON ip_addresses (country);
CREATE INDEX ip_addresses_reports_city ON ip_addresses (city);
CREATE INDEX ip_addresses_reports_region ON ip_addresses (region);
CREATE INDEX ip_addresses_reports_timezone ON ip_addresses (timezone);

CREATE TRIGGER trg_make_archive_of_changes_for_ip_addresses
    AFTER INSERT OR DELETE OR UPDATE
    ON invite_codes
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('IPAddresses');