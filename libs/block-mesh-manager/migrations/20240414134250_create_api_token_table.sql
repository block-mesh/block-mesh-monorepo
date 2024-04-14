CREATE TABLE api_tokens
(
    id         uuid PRIMARY KEY,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    user_id    uuid        NOT NULL,
    token      uuid        NOT NULL,
    status     text        not null,
    created_at timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)

);

CREATE INDEX api_tokens_created_at ON api_tokens (created_at);
CREATE INDEX api_tokens_status ON api_tokens (status);
CREATE INDEX api_tokens_user_id ON api_tokens (user_id);
CREATE INDEX api_tokens_token ON api_tokens (token);

CREATE TRIGGER trg_make_archive_of_changes_for_api_tokens
    AFTER INSERT OR DELETE OR UPDATE
    ON api_tokens
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('ApiToken');

