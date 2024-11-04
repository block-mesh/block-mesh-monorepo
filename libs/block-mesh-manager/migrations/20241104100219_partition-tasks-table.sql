ALTER TABLE tasks
    RENAME TO tasks_old;
-- -- -----
CREATE TABLE tasks
(
    id               uuid,
    user_id          uuid             NOT NULL,
    url              text             NOT NULL,
    method           text             NOT NULL,
    headers          jsonb,
    body             jsonb,
    assigned_user_id uuid,
    status           text             NOT NULL,
    response_code    integer,
    response_raw     text,
    created_at       timestamptz      NOT NULL,
    response_time    DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    retries_count    INTEGER          NOT NULL DEFAULT 0,
    asn              TEXT             NOT NULL DEFAULT '',
    colo             TEXT             NOT NULL DEFAULT '',
    country          TEXT             NOT NULL DEFAULT '',
    ip               TEXT             NOT NULL DEFAULT '',
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (id, status)
) PARTITION BY LIST (status);
-- -- -----
CREATE INDEX new_tasks_status ON tasks (status);
CREATE INDEX new_tasks_created_at ON tasks (created_at);
CREATE INDEX new_tasks_assigned_user_id_status ON tasks (assigned_user_id, status);
-- -- -----
CREATE TABLE tasks_pending PARTITION OF tasks FOR VALUES IN ('Pending');
CREATE TABLE tasks_assigned PARTITION OF tasks FOR VALUES IN ('Assigned');
CREATE TABLE tasks_completed PARTITION OF tasks FOR VALUES IN ('Completed');
CREATE TABLE tasks_failed PARTITION OF tasks FOR VALUES IN ('Failed');
-- -- -----
CREATE TABLE tasks_default PARTITION OF tasks DEFAULT;
-- -- -----
INSERT INTO tasks (id, user_id, url, method, headers, body, assigned_user_id, status, response_code, response_raw,
                   created_at, response_time, retries_count, asn, colo, country, ip)
SELECT id,
       user_id,
       url,
       method,
       headers,
       body,
       assigned_user_id,
       status,
       response_code,
       response_raw,
       created_at,
       response_time,
       retries_count,
       asn,
       colo,
       country,
       ip
FROM tasks_old;
-- -- -----
DROP TABLE tasks_old;