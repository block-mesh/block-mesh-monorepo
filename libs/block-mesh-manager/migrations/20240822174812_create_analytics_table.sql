CREATE TABLE analytics
(
    user_id          UUID        NOT NULL REFERENCES users (id),
    depin_aggregator TEXT        NOT NULL,
    device_type      TEXT        NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL,
    updated_at       TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (user_id, depin_aggregator)
);

