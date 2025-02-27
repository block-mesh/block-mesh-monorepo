ALTER TABLE twitter_tasks
    RENAME TO twitter_tasks_old;
------
CREATE TABLE twitter_tasks
(
    id               uuid                 default gen_random_uuid(),
    assigned_user_id uuid,
    twitter_username text        NOT NULL,
    status           text        NOT NULL,
    created_at       timestamptz NOT NULL default now(),
    updated_at       timestamptz NOT NULL default now(),
    since            date        NOT NULL,
    until            date        NOT NULL,
    delay            timestamptz NOT NULL,
    results          jsonb       not null,
    PRIMARY KEY (status, id)
) PARTITION BY LIST (status);
-----
CREATE INDEX twitter_tasks__assigned_user_id ON twitter_tasks (assigned_user_id);
CREATE INDEX twitter_tasks__twitter_username ON twitter_tasks (twitter_username);
CREATE INDEX twitter_tasks__status ON twitter_tasks (status);
CREATE INDEX twitter_tasks__created_at ON twitter_tasks (created_at);
CREATE INDEX twitter_tasks__updated_at ON twitter_tasks (updated_at);
CREATE INDEX twitter_tasks__since ON twitter_tasks (since);
CREATE INDEX twitter_tasks__until ON twitter_tasks (until);
CREATE INDEX twitter_tasks__delay ON twitter_tasks (delay);
CREATE UNIQUE INDEX twitter_tasks__twitter_username_since_until ON twitter_tasks (twitter_username, status, since, until);
-----
CREATE TABLE twitter_tasks_pending PARTITION OF twitter_tasks FOR VALUES IN ('Pending');
CREATE TABLE twitter_tasks_assigned PARTITION OF twitter_tasks FOR VALUES IN ('Assigned');
CREATE TABLE twitter_tasks_completed PARTITION OF twitter_tasks FOR VALUES IN ('Completed');
CREATE TABLE twitter_tasks_failed PARTITION OF twitter_tasks FOR VALUES IN ('Failed');
-----
CREATE TABLE twitter_tasks_default PARTITION OF twitter_tasks DEFAULT;
