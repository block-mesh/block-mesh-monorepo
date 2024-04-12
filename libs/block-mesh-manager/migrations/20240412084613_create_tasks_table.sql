CREATE TABLE tasks
(
    id               uuid PRIMARY KEY,
    sid              BIGSERIAL   NOT NULL UNIQUE,
    user_id          uuid        NOT NULL,
    url              text        NOT NULL,
    method           text        NOT NULL,
    headers          jsonb,
    body             jsonb,
    assigned_user_id uuid,
    status           text        NOT NULL,
    response_code    integer,
    response_raw     text,
    created_at       timestamptz NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX tasks_created_at ON tasks (created_at);
CREATE INDEX tasks_user_id ON tasks (user_id);
CREATE INDEX tasks_url ON tasks (url);
CREATE INDEX tasks_method ON tasks (method);
CREATE INDEX tasks_assigned_user_id ON tasks (assigned_user_id);
CREATE INDEX tasks_status ON tasks (status);
CREATE INDEX tasks_response_code ON tasks (response_code);

CREATE TRIGGER trg_make_archive_of_changes_for_tasks
    AFTER INSERT OR DELETE OR UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE FUNCTION make_archive_of_changes('Task');

