/*
SELECT
    DATE(created_at) AS day,
    COUNT(*) AS users_count
FROM
    users
GROUP BY
    DATE(created_at)
ORDER BY
    day DESC;

 */

/*
SELECT
    day,
    count(*) AS count
FROM
    daily_stats
GROUP BY
    day
ORDER BY
    day DESC
 */

/*
SELECT
    invited_by AS user_id,
    COUNT(*) AS invited_count
FROM
    users
WHERE
    invited_by IS NOT NULL
GROUP BY
    invited_by
ORDER BY
    invited_count DESC;
 */
