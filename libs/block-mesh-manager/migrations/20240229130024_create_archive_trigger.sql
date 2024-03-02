-- https://www.thegnar.com/blog/history-tracking-with-postgres

CREATE TABLE archives
(
    id          uuid               DEFAULT gen_random_uuid() PRIMARY KEY,
    sid         BIGSERIAL NOT NULL UNIQUE,
    table_name  TEXT      NOT NULL,
    record_type TEXT      NOT NULL,
    record_id   uuid      NOT NULL,
    operation   TEXT      NOT NULL,
    new_values  JSONB,
    old_values  JSONB,
    most_recent BOOLEAN   NOT NULL,
    created_at  timestamp NOT NULL DEFAULT NOW()
);

CREATE INDEX archives_table_name ON archives (table_name);
CREATE INDEX archives_record_type ON archives (record_type);
CREATE INDEX archives_record_id ON archives (record_id);
CREATE INDEX archives_operation ON archives (operation);
CREATE INDEX archives_created_at ON archives (created_at);
CREATE INDEX archives_most_recent ON archives (most_recent);
CREATE INDEX archives_table_name_most_recent ON archives (table_name, most_recent);

CREATE FUNCTION make_archive_of_changes() RETURNS TRIGGER
    LANGUAGE 'plpgsql'
AS
$$
    -- Expects one argument, the record_type
    -- It's the stringified ActiveRecord class name
    -- For example 'User', or 'Account'
BEGIN
    -- Previous snapshots should be marked as stale
    -- This little denormalization trick is so that ActiveRecord
    -- can immediately pull up the most recent snapshot without
    -- having to sort through all the records by their timestamps
    UPDATE archives
    SET most_recent = FALSE
    WHERE table_name = TG_TABLE_NAME
      AND most_recent = TRUE
      AND record_type = record_type
      AND record_id = (
        CASE
            WHEN TG_OP = 'DELETE'
                THEN OLD.id
            ELSE NEW.id
            END
        );


    IF TG_OP = 'INSERT' THEN
        INSERT INTO archives (table_name, record_type, record_id, operation, new_values, most_recent, created_at)
        VALUES (TG_TABLE_NAME, TG_ARGV[0], NEW.id, TG_OP, row_to_json(NEW), TRUE, now());
        RETURN NEW;

    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO archives (table_name, record_type, record_id, operation, new_values, old_values, most_recent,
                              created_at)
        VALUES (TG_TABLE_NAME, TG_ARGV[0], NEW.id, TG_OP, row_to_json(NEW), row_to_json(OLD), TRUE, now());
        RETURN NEW;

    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO archives (table_name, record_type, record_id, operation, old_values, most_recent, created_at)
        VALUES (TG_TABLE_NAME, TG_ARGV[0], OLD.id, TG_OP, row_to_json(OLD), TRUE, now());
        RETURN OLD;

    END IF;
END;
$$;