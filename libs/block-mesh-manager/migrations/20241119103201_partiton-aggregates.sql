ALTER TABLE aggregates
    RENAME TO aggregates_old;
-- -- -----
CREATE TABLE aggregates
(
    id               uuid        NOT NULL DEFAULT gen_random_uuid(),
    user_id          uuid        NOT NULL,
    name             TEXT        NOT NULL,
    value            JSONB       NOT NULL,
    created_at       timestamptz NOT NULL DEFAULT now(),
    updated_at       timestamptz NOT NULL DEFAULT now(),
    dummy_updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (name, id)
) PARTITION BY LIST (name);
-- -- -----
CREATE INDEX new_aggregates_id ON aggregates (id);
CREATE INDEX new_aggregates_status ON aggregates (name);
CREATE INDEX new_aggregates_created_at ON aggregates (created_at);
CREATE INDEX new_aggregates_updated_at ON aggregates (updated_at);
CREATE UNIQUE INDEX new_aggregates_assigned_user_id_name ON aggregates (user_id, name);
-- -- -----
CREATE TABLE aggregates_download PARTITION OF aggregates FOR VALUES IN ('Download');
CREATE TABLE aggregates_founder_twitter PARTITION OF aggregates FOR VALUES IN ('FounderTwitter');
CREATE TABLE aggregates_latency PARTITION OF aggregates FOR VALUES IN ('Latency');
CREATE TABLE aggregates_tasks PARTITION OF aggregates FOR VALUES IN ('Tasks');
CREATE TABLE aggregates_twitter PARTITION OF aggregates FOR VALUES IN ('Twitter');
CREATE TABLE aggregates_upload PARTITION OF aggregates FOR VALUES IN ('Upload');
CREATE TABLE aggregates_uptime PARTITION OF aggregates FOR VALUES IN ('Uptime');
-- -- -----
CREATE TABLE aggregates_default PARTITION OF aggregates DEFAULT;
-- -- -----
INSERT INTO aggregates (id, user_id, name, value, created_at, updated_at)
SELECT id,
       user_id,
       name,
       value,
       created_at,
       updated_at
FROM aggregates_old;
-- -- -----
DROP TABLE aggregates_old;