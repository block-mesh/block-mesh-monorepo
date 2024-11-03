use sqlx::migrate::{MigrateError, Migrator};
use sqlx::PgPool;
use std::env;
use std::path::Path;

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
    let path = env::current_dir()?;
    tracing::info!("Starting migrations from {}", path.to_string_lossy());

    for retry in 0..=opt.retry {
        if retry > 0 {
            tracing::info!("Retry number {} (waiting {}s)", retry, opt.retry_delay);
            std::thread::sleep(std::time::Duration::from_millis(opt.retry_delay));
        }
        let migrator = Migrator::new(Path::new("./migrations")).await?;

        for migration in migrator.iter() {
            tracing::info!("migration found = {:?}", migration.description);
        }

        // match sqlx::migrate!("./migrations").run(db_pool).await {
        match migrator.run(db_pool).await {
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
