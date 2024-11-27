CREATE TABLE emails
(
    id            uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id       uuid        NOT NULL,
    email_type    TEXT        NOT NULL,
    email_address TEXT        NOT NULL,
    message_id    TEXT        NOT NULL,
    created_at    timestamptz NOT NULL DEFAULT now(),
    updated_at    timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX emails_user_id ON emails (user_id);
CREATE INDEX emails_email_type ON emails (email_type);
CREATE INDEX emails_email_address ON emails (email_address);
CREATE INDEX emails_message_id ON emails (message_id);
CREATE INDEX emails_created_at ON emails (created_at);
CREATE INDEX emails_updated_at ON emails (updated_at);
