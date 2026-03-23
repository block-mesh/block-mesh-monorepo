ALTER TABLE users
    ADD COLUMN wallet_connected_sent BOOLEAN NOT NULL DEFAULT FALSE;

UPDATE users
SET extension_activated_sent = FALSE;

UPDATE users
SET wallet_connected_sent = FALSE;

CREATE INDEX IF NOT EXISTS users_wallet_connected_sent ON users (wallet_connected_sent);
