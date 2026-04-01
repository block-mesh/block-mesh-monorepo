# Purge Inactive Users

This is a one-off cleanup for the `block-mesh-manager` database.

## Scope

- Drops remaining archive-trigger overhead through normal app migrations.
- Purges users who are:
  - older than 180 days
  - `verified_email = FALSE`
  - `wallet_address IS NULL`
  - `proof_of_humanity IS NOT TRUE`
  - `role = 'user'`
  - absent from activity tables such as `uptime_reports`, `bandwidth_reports`, `users_ip`,
    `daily_stats`, `aggregates`, `analytics`, `perks`, `call_to_actions`,
    `daily_stats_background_jobs`, `tasks`, and `twitter_tasks`
- Preserves surviving referral trees by nulling `users.invited_by` before deleting matched users.

Rows in `nonces`, `api_tokens`, and `invite_codes` do not count as activity and are expected to
disappear through `ON DELETE CASCADE`.

## Order

1. Deploy the migrations first.
2. Run the preflight query and inspect the count plus sample rows.
3. Run the purge during a low-write window.
4. Confirm the candidate query comes back empty or much smaller afterward.

## Commands

```bash
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f scripts/db/purge_inactive_users_preflight.sql
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f scripts/db/purge_inactive_users.sql
psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f scripts/db/purge_inactive_users_preflight.sql
```

## Notes

- The purge runs in batches of `1000` users per transaction.
- If you need a different cutoff or batch size, edit `scripts/db/purge_inactive_users.sql`
  before running it.
- The procedure emits `NOTICE` lines with per-batch and total delete counts.
