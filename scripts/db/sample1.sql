INSERT
INTO aggregates (id, created_at, user_id, name, value, updated_at, dummy_updated_at)
VALUES ('11111111-1111-4111-8111-111111111111', now(), '11111111-1111-4111-8111-111111111111', 'Upload', '{
  "bla": "bla"
}'::jsonb, now(), now())
ON CONFLICT (user_id, name) DO UPDATE SET dummy_updated_at = now()
RETURNING id, created_at, user_id, name, value, updated_at