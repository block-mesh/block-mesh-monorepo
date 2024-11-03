WITH extant AS (SELECT id, created_at, user_id, name, value, updated_at
                FROM aggregates
                WHERE (user_id, name) = ('11111111-1111-4111-8111-111111111111', 'Upload')),
     inserted AS (
         INSERT INTO aggregates (id, created_at, user_id, name, value, updated_at, dummy_updated_at)
             SELECT '11111111-1111-4111-8111-111111111111',
                    now(),
                    '11111111-1111-4111-8111-111111111111',
                    'Upload',
                    '{
                      "bla": "bla"
                    }'::jsonb,
                    now(),
                    now()
             WHERE NOT EXISTS (SELECT FROM extant)
             RETURNING id, created_at, user_id, name, value, updated_at)
SELECT id, created_at, user_id, name, value, updated_at
FROM inserted
UNION ALL
SELECT id, created_at, user_id, name, value, updated_at
FROM extant