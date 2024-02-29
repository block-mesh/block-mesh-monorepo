use crate::configuration::application_settings::ApplicationSettings;
use crate::configuration::database_settings::DatabaseSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}
