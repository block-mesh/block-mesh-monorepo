ALTER TABLE users
    ADD COLUMN wallet_connected_sent BOOLEAN NOT NULL DEFAULT FALSE;

-- UPDATE users
-- SET extension_activated_sent = FALSE
-- WHERE extension_activated_sent = TRUE;

CREATE INDEX IF NOT EXISTS users_wallet_connected_sent ON users (wallet_connected_sent);
