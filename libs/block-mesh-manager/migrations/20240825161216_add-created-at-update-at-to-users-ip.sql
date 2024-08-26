ALTER TABLE users_ip ADD COLUMN created_at timestamptz NOT NULL DEFAULT now();
ALTER TABLE users_ip ADD COLUMN updated_at timestamptz NOT NULL DEFAULT now();

DROP INDEX users_ip_ip_id_user_id;
CREATE INDEX users_ip_created_at ON users_ip (created_at);
CREATE INDEX users_ip_updated_at ON users_ip (updated_at);
CREATE UNIQUE INDEX users_is_user_id_ip_id ON users_ip (user_id, ip_id);