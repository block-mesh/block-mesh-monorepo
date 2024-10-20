CREATE TABLE users
(
    id         uuid PRIMARY KEY,
    tg_id      BIGINT      NOT NULL UNIQUE,
    username   TEXT        NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX user_created_at ON users (created_at);
CREATE INDEX user_tg_id ON users (tg_id);
CREATE INDEX user_username ON users (username);