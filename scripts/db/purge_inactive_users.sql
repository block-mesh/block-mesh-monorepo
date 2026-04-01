SET application_name = 'purge_inactive_users';
SET lock_timeout = '5s';
SET statement_timeout = '0';

DROP PROCEDURE IF EXISTS purge_inactive_users_once(INTERVAL, INTEGER);

CREATE PROCEDURE purge_inactive_users_once(cutoff_interval INTERVAL, batch_size INTEGER)
LANGUAGE plpgsql
AS
$$
DECLARE
    selected_rows INTEGER;
    deleted_rows  INTEGER;
    total_rows    BIGINT := 0;
BEGIN
    IF batch_size < 1 THEN
        RAISE EXCEPTION 'batch_size must be at least 1';
    END IF;

    LOOP
        CREATE TEMP TABLE purge_inactive_users_batch ON COMMIT DROP AS
        SELECT u.id
        FROM users u
        WHERE u.created_at < now() - cutoff_interval
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
        ORDER BY u.created_at ASC, u.id ASC
        LIMIT batch_size;

        SELECT COUNT(*) INTO selected_rows
        FROM purge_inactive_users_batch;

        EXIT WHEN selected_rows = 0;

        UPDATE users
        SET invited_by = NULL
        WHERE invited_by IN (SELECT id FROM purge_inactive_users_batch);

        DELETE FROM users
        WHERE id IN (SELECT id FROM purge_inactive_users_batch);

        GET DIAGNOSTICS deleted_rows = ROW_COUNT;
        total_rows := total_rows + deleted_rows;

        RAISE NOTICE 'deleted % inactive users in this batch (% total)', deleted_rows, total_rows;

        COMMIT;
    END LOOP;

    RAISE NOTICE 'inactive user purge complete, deleted % users total', total_rows;
END;
$$;

CALL purge_inactive_users_once(INTERVAL '180 days', 1000);

DROP PROCEDURE purge_inactive_users_once(INTERVAL, INTEGER);

ANALYZE users;
ANALYZE aggregates;
ANALYZE analytics;
ANALYZE bandwidth_reports;
ANALYZE daily_stats;
ANALYZE daily_stats_background_jobs;
ANALYZE tasks;
ANALYZE twitter_tasks;
ANALYZE uptime_reports;
ANALYZE users_ip;
