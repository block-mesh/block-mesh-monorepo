CREATE TABLE proxy_masters
(
    id         uuid PRIMARY KEY,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    address    TEXT        NOT NULL UNIQUE,
    status     TEXT        NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX provider_node_created_at ON proxy_masters (created_at);
CREATE INDEX provider_node_address ON proxy_masters (address);
CREATE INDEX provider_node_status ON proxy_masters (status);

CREATE TRIGGER trg_make_archive_of_changes_for_proxy_masters
    AFTER INSERT OR DELETE OR UPDATE
    ON proxy_masters
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('ProviderNode');

