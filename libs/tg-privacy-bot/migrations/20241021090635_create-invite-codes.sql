CREATE TABLE invite_codes
(
    id          uuid PRIMARY KEY,
    invite_code TEXT        NOT NULL,
    user_id     uuid        NOT NULL,
    created_at  timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX invite_codes_created_at ON invite_codes (created_at);
CREATE INDEX invite_codes_user_id ON invite_codes (user_id);
CREATE UNIQUE INDEX invite_codes_invite_code ON invite_codes (invite_code);
