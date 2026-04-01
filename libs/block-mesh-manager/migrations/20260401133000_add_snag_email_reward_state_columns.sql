ALTER TABLE users
    ADD COLUMN snag_email_reward_pending BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN snag_email_reward_consumed BOOLEAN NOT NULL DEFAULT FALSE;
