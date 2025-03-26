CREATE TABLE ids
(
    id         uuid PRIMARY KEY,
    email      TEXT        NOT NULL,
    api_token  TEXT        NOT NULL,
    fp         TEXT        NOT NULL,
    ip         TEXT        NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX id_ids on ids (id);
CREATE INDEX email_ids on ids (email);
CREATE INDEX api_token_ids on ids (api_token);
CREATE INDEX fp_ids on ids (fp);
CREATE INDEX ip_ids on ids (ip);
CREATE UNIQUE INDEX all_ids on ids (email, api_token, fp, ip);