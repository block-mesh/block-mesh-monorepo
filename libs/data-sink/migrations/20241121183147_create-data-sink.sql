CREATE TABLE data_sinks
(
    id         uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id    uuid        NOT NULL,
    origin     TEXT        NOT NULL,
    origin_id  TEXT        NOT NULL,
    user_name  TEXT        NOT NULL,
    link       TEXT        NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    raw        TEXT        NOT NULL
);

CREATE INDEX data_sinks_created_at ON data_sinks (created_at);
CREATE INDEX data_sinks_updated_at ON data_sinks (updated_at);
CREATE INDEX data_sinks_user_id ON data_sinks (user_id);
CREATE UNIQUE INDEX data_sinks_origin_origin_id ON data_sinks (origin, origin_id);

