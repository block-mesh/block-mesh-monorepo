CREATE table rpcs (
    id        uuid          PRIMARY KEY,
    sid       BIGSERIAL     NOT NULL UNIQUE,
    token     TEXT          NOT NULL,
    name      TEXT          NOT NULL,
    host      TEXT          NOT NULL,
    created_at timestamptz  NOT NULL
);

CREATE INDEX rpcs_created_at ON rpcs (created_at);
CREATE INDEX rpcs_token ON rpcs (token);
CREATE INDEX rpcs_name ON rpcs (name);
CREATE INDEX rpcs_host ON rpcs (host);

CREATE TRIGGER trg_make_archive_of_changes_for_rpcs
    AFTER INSERT OR DELETE OR UPDATE
    ON rpcs
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Rpc');