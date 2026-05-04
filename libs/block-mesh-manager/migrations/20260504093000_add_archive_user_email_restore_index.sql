-- no-transaction

CREATE INDEX CONCURRENTLY IF NOT EXISTS archives_users_most_recent_email_created_at
ON archives (
    (lower(COALESCE(new_values, old_values) ->> 'email')),
    created_at DESC
)
WHERE table_name = 'users'
  AND record_type = 'User'
  AND most_recent = TRUE;
