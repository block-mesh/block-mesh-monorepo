INSERT INTO users
    (id, email, password, created_at)
VALUES
    (gen_random_uuid(), 'support+111@blockmesh.xyz', gen_random_uuid()::text, now())
ON CONFLICT (email) DO NOTHING;


INSERT INTO invite_codes
    (id, invite_code, user_id, created_at)
VALUES
    (gen_random_uuid(), '123', (SELECT id from users where email = 'support+111@blockmesh.xyz' LIMIT 1), now())
ON CONFLICT(invite_code) DO NOTHING;
