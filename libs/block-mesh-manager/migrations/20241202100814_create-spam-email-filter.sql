CREATE TABLE spam_emails
(
    id         uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    domain     TEXT        NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX spam_emails_domain on spam_emails (domain);