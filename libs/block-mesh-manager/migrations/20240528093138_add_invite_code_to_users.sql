ALTER TABLE users ADD invited_by uuid;
CREATE INDEX user_invited_by ON users (invited_by);