ALTER TABLE perks ALTER COLUMN name SET NOT NULL;

CREATE TABLE call_to_actions
(
    id              uuid PRIMARY KEY,
    sid             BIGSERIAL NOT NULL UNIQUE,
    user_id         uuid NOT NULL,
    created_at      timestamptz NOT NULL,
    name            TEXT NOT NULL,
    status          bool NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX call_to_action_created_at ON call_to_actions (created_at);
CREATE INDEX call_to_action_name ON call_to_actions (name);
CREATE INDEX call_to_action_user_id ON call_to_actions (user_id);
CREATE UNIQUE INDEX call_to_action_user_id_name ON call_to_actions (user_id, name);

CREATE TRIGGER trg_make_archive_of_changes_for_call_to_action
    AFTER INSERT OR DELETE OR UPDATE
    ON call_to_actions
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('CallToAction');
