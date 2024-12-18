CREATE TABLE claims
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    user_id     uuid             NOT NULL,
    created_at  timestamptz      NOT NULL DEFAULT now(),
    updated_at  timestamptz      NOT NULL DEFAULT now(),
    batch       TEXT             NOT NULL,
    distributed BOOLEAN          NOT NULL DEFAULT FALSE,
    sent_to     TEXT,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE INDEX claims_user_id ON claims (user_id);
CREATE INDEX claims_batch ON claims (batch);
CREATE INDEX claims_distributed ON claims (distributed);
CREATE INDEX claims_sent_to ON claims (sent_to);
CREATE UNIQUE INDEX claims_user_id_batch ON claims (batch, user_id);