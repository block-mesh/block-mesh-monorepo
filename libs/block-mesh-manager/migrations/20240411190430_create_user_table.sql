CREATE TABLE users
(
    id             uuid PRIMARY KEY,
    sid            BIGSERIAL   NOT NULL UNIQUE,
    email          TEXT        NOT NULL UNIQUE,
    password       TEXT        NOT NULL,
    wallet_address TEXT UNIQUE,
    created_at     timestamptz NOT NULL
);

CREATE INDEX user_created_at ON users (created_at);
CREATE INDEX user_email ON users (email);

CREATE TRIGGER trg_make_archive_of_changes_for_users
    AFTER INSERT OR DELETE OR UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('User');

