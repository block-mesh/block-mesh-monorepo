CREATE TABLE nonces
(
    id         uuid PRIMARY KEY,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    nonce      TEXT        NOT NULL,
    user_id    uuid        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX nonces_created_at ON nonces (created_at);
CREATE INDEX nonces_nonce ON nonces (nonce);
CREATE INDEX nonces_wallet_address ON nonces (user_id);

CREATE TRIGGER trg_make_archive_of_changes_for_nonces
    AFTER INSERT OR DELETE OR UPDATE
    ON nonces
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Nonce');

