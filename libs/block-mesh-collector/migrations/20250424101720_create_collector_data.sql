CREATE TABLE collector_datas
(
    id         uuid PRIMARY KEY,
    created_at timestamptz NOT NULL,
    source     TEXT        NOT NULL,
    data       JSONB       NOT NULL
);

CREATE INDEX collector_datas_created_at ON collector_datas (created_at);
CREATE INDEX collector_datas_source ON collector_datas (source);