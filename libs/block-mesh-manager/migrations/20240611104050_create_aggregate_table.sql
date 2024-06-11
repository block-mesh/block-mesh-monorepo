CREATE TABLE aggregates
(
    id        uuid PRIMARY KEY,
    sid       BIGSERIAL   NOT NULL UNIQUE,
    user_id   uuid        NOT NULL,
    name      TEXT        NOT NULL,
    value     JSONB       NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX aggregates_created_at ON aggregates (created_at);
CREATE INDEX aggregates_user_id ON aggregates (user_id);
CREATE INDEX aggregates_name ON aggregates (name);
CREATE UNIQUE INDEX aggregates_user_id_name ON aggregates (user_id, name);

CREATE TRIGGER trg_make_archive_of_changes_for_aggregates
    AFTER INSERT OR DELETE OR UPDATE
    ON aggregates
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Aggregate');