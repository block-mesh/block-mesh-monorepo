ALTER TABLE tasks ADD COLUMN country TEXT NOT NULL DEFAULT '';
ALTER TABLE tasks ADD COLUMN ip TEXT NOT NULL DEFAULT '';
ALTER TABLE tasks ADD COLUMN asn TEXT NOT NULL DEFAULT '';
ALTER TABLE tasks ADD COLUMN colo TEXT NOT NULL DEFAULT '';
ALTER TABLE tasks ADD COLUMN response_time DOUBLE PRECISION NOT NULL DEFAULT 0.0;

CREATE INDEX tasks_country ON tasks (country);
CREATE INDEX tasks_ip ON tasks (ip);
CREATE INDEX tasks_asn ON tasks (asn);
CREATE INDEX tasks_colo ON tasks (colo);
CREATE INDEX response_time ON tasks (response_time);

