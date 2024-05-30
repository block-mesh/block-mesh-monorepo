-- Add migration script here
ALTER TABLE users ADD COLUMN verified_email BOOLEAN NOT NULL DEFAULT FALSE;
