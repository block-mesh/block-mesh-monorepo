-- Add migration script here
ALTER TABLE perks ADD COLUMN one_time_bonus DOUBLE PRECISION NOT NULL DEFAULT 0;

CREATE INDEX perks_one_time_bonus ON perks (one_time_bonus);
