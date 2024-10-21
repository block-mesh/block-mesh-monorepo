CREATE TABLE user_settings
(
    id           uuid PRIMARY KEY,
    user_id      uuid        NOT NULL UNIQUE,
    message_mode TEXT        NOT NULL,
    model_name   TEXT        NOT NULL,
    created_at   timestamptz NOT NULL,
    updated_at   timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX user_settings_created_at ON user_settings (created_at);
CREATE INDEX user_settings_updated_at ON user_settings (updated_at);
CREATE INDEX user_settings_message_mode ON user_settings (message_mode);
CREATE INDEX user_settings_model ON user_settings (model_name);