ALTER TABLE users
    ADD COLUMN extension_activated_sent BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX IF NOT EXISTS users_extension_activated_sent ON users (extension_activated_sent);
