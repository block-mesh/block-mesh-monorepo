CREATE TABLE analytics
(
    id               uuid PRIMARY KEY,
    user_id          UUID NOT NULL,
    depin_aggregator TEXT NOT NULL,
    device_type      TEXT        NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL,
    updated_at       TIMESTAMPTZ NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX analytics_user_id_depin_aggregator ON analytics (user_id, depin_aggregator);
CREATE INDEX analytics_user_id ON analytics (user_id);
CREATE INDEX analytics_device_type ON analytics (device_type);
CREATE INDEX analytics_depin_aggregator ON analytics (depin_aggregator);
CREATE INDEX analytics_depin_created_at ON analytics (created_at);
CREATE INDEX analytics_depin_updated_at ON analytics (updated_at);

CREATE TRIGGER trg_make_archive_of_changes_for_analytics
    AFTER INSERT OR DELETE OR UPDATE
    ON analytics
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Analytics');
