use sqlx::migrate::{MigrateError, Migrator};
use sqlx::PgPool;
use std::env;
use std::path::Path;
use std::time::Duration;

struct Migrate {
    /// It will retry this number of times before giving up
    pub retry: u64,
    /// Make migration sleep this amount of time before each retry
    pub retry_delay_ms: u64,
}

#[tracing::instrument(name = "migrate", skip_all, ret, err)]
pub async fn migrate(db_pool: &PgPool, env: String) -> anyhow::Result<()> {
    let opt = Migrate {
        retry: 3,
        retry_delay_ms: 1_000,
    };
    let path = env::current_dir()?;
    tracing::info!("Starting migrations from {}", path.to_string_lossy());
    let migrator = Migrator::new(Path::new("./migrations")).await?;

    for migration in migrator.iter() {
        tracing::info!("migration found = {:?}", migration.description);
    }

    for retry in 0..=opt.retry {
        if retry > 0 {
            tracing::warn!(
                "Retrying migrations after transient failure, attempt {} of {} (waiting {}ms)",
                retry,
                opt.retry,
                opt.retry_delay_ms
            );
            tokio::time::sleep(Duration::from_millis(opt.retry_delay_ms)).await;
        }

        let mut conn = db_pool.acquire().await?;
        prepare_migration_connection(&mut conn).await?;

        match migrator.run_direct(&mut *conn).await {
            Ok(_) => tracing::info!("Successfully migrated"),
            Err(MigrateError::VersionMissing(version)) if env != "local" => {
                tracing::warn!("Skipping missing migration version {}", version);
                return Ok(());
            }
            Err(e) if retry < opt.retry && is_retryable_migration_error(&e) => {
                tracing::warn!("Migration attempt failed with retryable error: {}", e);
                continue;
            }
            Err(e) => {
                if env == "local" {
                    tracing::warn!("Ignoring migration failure in local environment: {}", e);
                } else {
                    tracing::error!("Failed to migrate database: {}", e);
                    return Err(e.into());
                }
            }
        }
        tracing::info!("Migration completed with success");
        return Ok(());
    }

    Ok(())
}

async fn prepare_migration_connection(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> anyhow::Result<()> {
    let statement_timeout = env::var("statement_timeout_migration").unwrap_or("0".to_string());
    let lock_timeout = env::var("lock_timeout_migration").unwrap_or("0".to_string());

    sqlx::query("SELECT set_config('statement_timeout', $1, false)")
        .bind(&statement_timeout)
        .fetch_optional(conn.as_mut())
        .await?;
    sqlx::query("SELECT set_config('lock_timeout', $1, false)")
        .bind(&lock_timeout)
        .fetch_optional(conn.as_mut())
        .await?;

    tracing::info!(
        statement_timeout = %statement_timeout,
        lock_timeout = %lock_timeout,
        "Configured migration session timeouts"
    );

    Ok(())
}

fn is_retryable_migration_error(error: &MigrateError) -> bool {
    let is_lock_timeout = match error {
        MigrateError::Execute(sqlx_error) | MigrateError::ExecuteMigration(sqlx_error, _) => {
            sqlx_error
                .as_database_error()
                .and_then(|db_error| db_error.code())
                .is_some_and(|code| code.as_ref() == "55P03")
        }
        _ => false,
    };

    is_lock_timeout || error.to_string().contains("lock timeout")
}
