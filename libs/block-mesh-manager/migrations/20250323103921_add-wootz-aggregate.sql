CREATE TABLE aggregates_wootz PARTITION OF aggregates FOR VALUES IN ('Wootz');
CREATE TABLE aggregates_interactive_ext PARTITION OF aggregates FOR VALUES IN ('InteractiveExt');
----------------------------
INSERT INTO aggregates (id, user_id, name, value, created_at, updated_at)
SELECT gen_random_uuid() AS id,
       u.id              AS user_id,
       'Wootz'           AS name,
       '0'::jsonb        AS value,
       NOW()             AS created_at,
       NOW()             AS updated_at
FROM users u
         LEFT JOIN aggregates a
                   ON a.user_id = u.id
                       AND a.name = 'Wootz'
WHERE a.id IS NULL
ON CONFLICT (user_id, name) DO NOTHING;
----------------------------
INSERT INTO aggregates (id, user_id, name, value, created_at, updated_at)
SELECT gen_random_uuid() AS id,
       u.id              AS user_id,
       'InteractiveExt'  AS name,
       '0'::jsonb        AS value,
       NOW()             AS created_at,
       NOW()             AS updated_at
FROM users u
         LEFT JOIN aggregates a
                   ON a.user_id = u.id
                       AND a.name = 'InteractiveExt'
WHERE a.id IS NULL
ON CONFLICT (user_id, name) DO NOTHING;