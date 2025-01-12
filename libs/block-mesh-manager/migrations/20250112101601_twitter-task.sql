CREATE TABLE twitter_tasks
(
    id               uuid primary key     default gen_random_uuid(),
    assigned_user_id uuid,
    twitter_username text        NOT NULL,
    status           text        NOT NULL,
    created_at       timestamptz NOT NULL default now(),
    updated_at       timestamptz NOT NULL default now(),
    since            date        NOT NULL,
    until            date        NOT NULL,
    delay            timestamptz NOT NULL,
    results          jsonb       not null
);


CREATE INDEX twitter_tasks_assigned_user_id ON twitter_tasks (assigned_user_id);
CREATE INDEX twitter_tasks_twitter_username ON twitter_tasks (twitter_username);
CREATE INDEX twitter_tasks_status ON twitter_tasks (status);
CREATE INDEX twitter_tasks_created_at ON twitter_tasks (created_at);
CREATE INDEX twitter_tasks_updated_at ON twitter_tasks (updated_at);
CREATE INDEX twitter_tasks_since ON twitter_tasks (since);
CREATE INDEX twitter_tasks_until ON twitter_tasks (until);
CREATE INDEX twitter_tasks_delay ON twitter_tasks (delay);
CREATE UNIQUE INDEX twitter_tasks_twitter_username_since_until ON twitter_tasks (twitter_username, since, until);
