ALTER TABLE aggregates
    DROP CONSTRAINT fk_user;
ALTER TABLE aggregates
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE analytics
    DROP CONSTRAINT fk_user;
ALTER TABLE analytics
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE api_tokens
    DROP CONSTRAINT fk_user;
ALTER TABLE api_tokens
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE bandwidth_reports
    DROP CONSTRAINT fk_user;
ALTER TABLE bandwidth_reports
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE call_to_actions
    DROP CONSTRAINT fk_user;
ALTER TABLE call_to_actions
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE daily_stats
    DROP CONSTRAINT fk_user;
ALTER TABLE daily_stats
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE invite_codes
    DROP CONSTRAINT fk_user;
ALTER TABLE invite_codes
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE nonces
    DROP CONSTRAINT fk_user;
ALTER TABLE nonces
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE perks
    DROP CONSTRAINT fk_user;
ALTER TABLE perks
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE tasks
    DROP CONSTRAINT fk_user;
ALTER TABLE tasks
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE uptime_reports
    DROP CONSTRAINT fk_user;
ALTER TABLE uptime_reports
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE users_ip
    DROP CONSTRAINT fk_ip;
ALTER TABLE users_ip
    ADD CONSTRAINT fk_ip FOREIGN KEY (ip_id) REFERENCES ip_addresses (id) ON DELETE CASCADE;

ALTER TABLE users_ip
    DROP CONSTRAINT fk_user;
ALTER TABLE users_ip
    ADD CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;