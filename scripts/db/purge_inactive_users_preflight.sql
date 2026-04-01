SET application_name = 'purge_inactive_users_preflight';

WITH inactive_user_candidates AS MATERIALIZED (
    SELECT
        u.id,
        u.email,
        u.created_at,
        u.invited_by
    FROM users u
    WHERE u.created_at < now() - INTERVAL '180 days'
      AND u.verified_email = FALSE
      AND u.wallet_address IS NULL
      AND COALESCE(u.proof_of_humanity, FALSE) = FALSE
      AND u.role = 'user'
      AND NOT EXISTS (
        SELECT 1
        FROM uptime_reports ur
        WHERE ur.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM bandwidth_reports br
        WHERE br.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM users_ip ui
        WHERE ui.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM daily_stats ds
        WHERE ds.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM aggregates a
        WHERE a.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM analytics an
        WHERE an.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM perks p
        WHERE p.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM call_to_actions cta
        WHERE cta.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM daily_stats_background_jobs dsbj
        WHERE dsbj.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM tasks t
        WHERE t.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM tasks t_assigned
        WHERE t_assigned.assigned_user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM twitter_tasks tt
        WHERE tt.assigned_user_id = u.id
    )
)
SELECT
    COUNT(*) AS candidate_count,
    MIN(created_at) AS oldest_candidate_created_at,
    MAX(created_at) AS newest_candidate_created_at
FROM inactive_user_candidates;

WITH inactive_user_candidates AS MATERIALIZED (
    SELECT
        u.id,
        u.email,
        u.created_at,
        u.invited_by
    FROM users u
    WHERE u.created_at < now() - INTERVAL '180 days'
      AND u.verified_email = FALSE
      AND u.wallet_address IS NULL
      AND COALESCE(u.proof_of_humanity, FALSE) = FALSE
      AND u.role = 'user'
      AND NOT EXISTS (
        SELECT 1
        FROM uptime_reports ur
        WHERE ur.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM bandwidth_reports br
        WHERE br.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM users_ip ui
        WHERE ui.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM daily_stats ds
        WHERE ds.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM aggregates a
        WHERE a.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM analytics an
        WHERE an.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM perks p
        WHERE p.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM call_to_actions cta
        WHERE cta.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM daily_stats_background_jobs dsbj
        WHERE dsbj.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM tasks t
        WHERE t.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM tasks t_assigned
        WHERE t_assigned.assigned_user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM twitter_tasks tt
        WHERE tt.assigned_user_id = u.id
    )
)
SELECT
    id,
    email,
    created_at,
    invited_by
FROM inactive_user_candidates
ORDER BY created_at ASC, id ASC
LIMIT 50;

WITH inactive_user_candidates AS MATERIALIZED (
    SELECT u.id
    FROM users u
    WHERE u.created_at < now() - INTERVAL '180 days'
      AND u.verified_email = FALSE
      AND u.wallet_address IS NULL
      AND COALESCE(u.proof_of_humanity, FALSE) = FALSE
      AND u.role = 'user'
      AND NOT EXISTS (
        SELECT 1
        FROM uptime_reports ur
        WHERE ur.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM bandwidth_reports br
        WHERE br.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM users_ip ui
        WHERE ui.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM daily_stats ds
        WHERE ds.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM aggregates a
        WHERE a.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM analytics an
        WHERE an.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM perks p
        WHERE p.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM call_to_actions cta
        WHERE cta.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM daily_stats_background_jobs dsbj
        WHERE dsbj.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM tasks t
        WHERE t.user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM tasks t_assigned
        WHERE t_assigned.assigned_user_id = u.id
    )
      AND NOT EXISTS (
        SELECT 1
        FROM twitter_tasks tt
        WHERE tt.assigned_user_id = u.id
    )
)
SELECT COUNT(*) AS surviving_users_pointing_to_deleted_inviters
FROM users u
WHERE u.invited_by IN (SELECT id FROM inactive_user_candidates);
