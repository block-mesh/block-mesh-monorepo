use sqlx::migrate::MigrateError;
use sqlx::PgPool;

struct Migrate {
    /// It will retry this number of times before giving up
    pub retry: u64,
    /// Make migration sleep this amount of time before each retry
    pub retry_delay: u64,
}

#[tracing::instrument(name = "migrate", skip_all, ret, err)]
pub async fn migrate(db_pool: &PgPool, env: String) -> anyhow::Result<()> {
    let opt = Migrate {
        retry: 3,
        retry_delay: 100,
    };
    tracing::info!("Starting migrations");

    for retry in 0..=opt.retry {
        if retry > 0 {
            tracing::info!("Retry number {} (waiting {}s)", retry, opt.retry_delay);
            std::thread::sleep(std::time::Duration::from_millis(opt.retry_delay));
        }
        match sqlx::migrate!("./migrations").run(db_pool).await {
            Ok(_) => tracing::info!("Successfully migrated"),
            Err(e) => {
                if env != "local" {
                    match e {
                        MigrateError::VersionMissing(_) => {
                            tracing::warn!("Failed to migrate database: {}", e);
                        }
                        _ => {
                            tracing::error!("Failed to migrate database: {}", e);
                            return Err(e.into());
                        }
                    }
                }
            }
        }
        tracing::info!("Migration completed with success");
    }
    Ok(())
}
