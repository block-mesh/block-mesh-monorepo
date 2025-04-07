CREATE TABLE rama_ids
(
    id         uuid PRIMARY KEY,
    email      TEXT        NOT NULL,
    api_token  TEXT        NOT NULL,
    ja3        TEXT        NOT NULL,
    ja4        TEXT        NOT NULL,
    ja4h       TEXT        NOT NULL,
    ip         TEXT        NOT NULL,
    created_at timestamptz NOT NULL
);

CREATE INDEX id_rama_ids on rama_ids (id);
CREATE INDEX email_rama_ids on rama_ids (email);
CREATE INDEX api_token_rama_ids on rama_ids (api_token);
CREATE INDEX ja3_rama_ids on rama_ids (ja3);
CREATE INDEX ja4_rama_ids on rama_ids (ja4);
CREATE INDEX ja4h_rama_ids on rama_ids (ja4h);
CREATE INDEX ip_rama_ids on rama_ids (ip);
CREATE UNIQUE INDEX all_ids on rama_ids (email, api_token, ip, ja4, ja4h, ja3);