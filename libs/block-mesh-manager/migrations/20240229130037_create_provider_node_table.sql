CREATE TABLE provider_nodes
(
    id         uuid PRIMARY KEY,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    address    TEXT        NOT NULL UNIQUE,
    status     TEXT        NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX provider_node_created_at ON provider_nodes (created_at);
CREATE INDEX provider_node_address ON provider_nodes (address);
CREATE INDEX provider_node_status ON provider_nodes (status);

CREATE TRIGGER trg_make_archive_of_changes_for_provider_nodes
    AFTER INSERT OR DELETE OR UPDATE
    ON provider_nodes
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('ProviderNode');

