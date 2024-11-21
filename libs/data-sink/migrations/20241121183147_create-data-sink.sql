CREATE TABLE data_sinks
(
    id         uuid PRIMARY KEY,
    user_id    uuid        NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX data_sinks_created_at ON data_sinks (created_at);
CREATE INDEX data_sinks_updated_at ON data_sinks (updated_at);
CREATE INDEX data_sinks_user_id ON data_sinks (user_id);

