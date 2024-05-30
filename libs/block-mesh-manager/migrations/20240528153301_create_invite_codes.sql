CREATE TABLE invite_codes
(
    id         uuid PRIMARY KEY,
    sid        BIGSERIAL   NOT NULL UNIQUE,
    invite_code TEXT        NOT NULL,
    user_id    uuid        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX invite_codes_created_at ON invite_codes (created_at);
CREATE INDEX invite_codes_user_id ON invite_codes (user_id);
CREATE UNIQUE INDEX invite_codes_invite_code ON invite_codes (invite_code);

CREATE TRIGGER trg_make_archive_of_changes_for_invite_codes
    AFTER INSERT OR DELETE OR UPDATE
    ON invite_codes
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('InviteCode');

