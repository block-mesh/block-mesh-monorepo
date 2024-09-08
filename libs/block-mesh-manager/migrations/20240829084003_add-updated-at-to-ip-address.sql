ALTER TABLE ip_addresses ADD COLUMN updated_at timestamptz NOT NULL DEFAULT now();
CREATE INDEX ip_addresses_updated_at ON ip_addresses (updated_at);