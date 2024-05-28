ALTER TABLE users ADD invited_by uuid;
ALTER TABLE users ADD invite_code text NOT NULL DEFAULT '';

CREATE INDEX user_invited_by ON users (invited_by);
CREATE UNIQUE INDEX user_invite_code ON users (invite_code);