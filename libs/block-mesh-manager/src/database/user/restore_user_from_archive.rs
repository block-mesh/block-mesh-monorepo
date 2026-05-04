use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreUserFromArchiveResult {
    Restored(Uuid),
    AlreadyExists,
    ArchiveNotFound,
    Conflict(RestoreUserFromArchiveConflict),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreUserFromArchiveConflict {
    pub field: RestoreUserFromArchiveConflictField,
    pub value: Option<String>,
    pub conflicting_user_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreUserFromArchiveConflictField {
    WalletAddress,
    Unknown,
}

impl RestoreUserFromArchiveConflictField {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WalletAddress => "wallet_address",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug)]
struct RestoreUserFromArchiveRow {
    restored_id: Option<Uuid>,
    archive_found: bool,
    email_exists: bool,
    id_exists: bool,
    wallet_exists: bool,
    archived_wallet_address: Option<String>,
    wallet_conflicting_user_id: Option<Uuid>,
}

pub async fn restore_user_from_archive(
    transaction: &mut Transaction<'_, Postgres>,
    email: &str,
) -> Result<RestoreUserFromArchiveResult, sqlx::Error> {
    let row = sqlx::query_as!(
        RestoreUserFromArchiveRow,
        r#"
        WITH archived_user AS (
            SELECT COALESCE(a.new_values, a.old_values) AS snapshot
            FROM archives a
            WHERE a.table_name = 'users'
              AND a.record_type = 'User'
              AND a.most_recent = TRUE
              AND lower(COALESCE(a.new_values, a.old_values) ->> 'email') = $1
            ORDER BY a.created_at DESC
            LIMIT 1
        ),
        inserted AS (
            INSERT INTO users (
                id,
                email,
                password,
                wallet_address,
                created_at,
                role,
                invited_by,
                verified_email,
                status,
                comments,
                proof_of_humanity,
                extension_activated,
                extension_activated_sent,
                wallet_connected_sent,
                email_confirmed_sent,
                snag_email_reward_pending,
                snag_email_reward_consumed
            )
            SELECT
                (snapshot ->> 'id')::uuid,
                snapshot ->> 'email',
                snapshot ->> 'password',
                NULLIF(snapshot ->> 'wallet_address', ''),
                COALESCE((snapshot ->> 'created_at')::timestamptz, now()),
                COALESCE(snapshot ->> 'role', 'user'),
                NULLIF(snapshot ->> 'invited_by', '')::uuid,
                COALESCE((snapshot ->> 'verified_email')::boolean, FALSE),
                snapshot ->> 'status',
                snapshot ->> 'comments',
                COALESCE((snapshot ->> 'proof_of_humanity')::boolean, FALSE),
                COALESCE((snapshot ->> 'extension_activated')::boolean, FALSE),
                COALESCE((snapshot ->> 'extension_activated_sent')::boolean, FALSE),
                COALESCE((snapshot ->> 'wallet_connected_sent')::boolean, FALSE),
                COALESCE((snapshot ->> 'email_confirmed_sent')::boolean, FALSE),
                COALESCE((snapshot ->> 'snag_email_reward_pending')::boolean, FALSE),
                COALESCE((snapshot ->> 'snag_email_reward_consumed')::boolean, FALSE)
            FROM archived_user
            ON CONFLICT DO NOTHING
            RETURNING id
        )
        SELECT
            (SELECT id FROM inserted LIMIT 1) AS "restored_id?",
            EXISTS(SELECT 1 FROM archived_user) AS "archive_found!",
            EXISTS(
                SELECT 1
                FROM users u
                JOIN archived_user au ON u.email = au.snapshot ->> 'email'
            ) AS "email_exists!",
            EXISTS(
                SELECT 1
                FROM users u
                JOIN archived_user au ON u.id = (au.snapshot ->> 'id')::uuid
            ) AS "id_exists!",
            EXISTS(
                SELECT 1
                FROM users u
                JOIN archived_user au
                  ON NULLIF(au.snapshot ->> 'wallet_address', '') IS NOT NULL
                 AND u.wallet_address = NULLIF(au.snapshot ->> 'wallet_address', '')
            ) AS "wallet_exists!",
            (
                SELECT NULLIF(au.snapshot ->> 'wallet_address', '')
                FROM archived_user au
                LIMIT 1
            ) AS "archived_wallet_address?",
            (
                SELECT u.id
                FROM users u
                JOIN archived_user au
                  ON NULLIF(au.snapshot ->> 'wallet_address', '') IS NOT NULL
                 AND u.wallet_address = NULLIF(au.snapshot ->> 'wallet_address', '')
                LIMIT 1
            ) AS "wallet_conflicting_user_id?"
        "#,
        email,
    )
    .fetch_one(&mut **transaction)
    .await?;

    Ok(match row.restored_id {
        Some(user_id) => RestoreUserFromArchiveResult::Restored(user_id),
        None if !row.archive_found => RestoreUserFromArchiveResult::ArchiveNotFound,
        None if row.email_exists || row.id_exists => RestoreUserFromArchiveResult::AlreadyExists,
        None if row.wallet_exists => {
            RestoreUserFromArchiveResult::Conflict(RestoreUserFromArchiveConflict {
                field: RestoreUserFromArchiveConflictField::WalletAddress,
                value: row.archived_wallet_address,
                conflicting_user_id: row.wallet_conflicting_user_id,
            })
        }
        None => RestoreUserFromArchiveResult::Conflict(RestoreUserFromArchiveConflict {
            field: RestoreUserFromArchiveConflictField::Unknown,
            value: None,
            conflicting_user_id: None,
        }),
    })
}
