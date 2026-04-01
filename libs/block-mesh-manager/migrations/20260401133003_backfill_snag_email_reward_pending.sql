UPDATE users
SET snag_email_reward_pending = TRUE
WHERE created_at >= TIMESTAMPTZ '2026-03-24 00:00:00+00'
  AND snag_email_reward_consumed = FALSE;
