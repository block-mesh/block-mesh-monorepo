use leptos::*;

use block_mesh_common::app_config::AppConfig;
use block_mesh_common::cli::CommandsEnum;

#[derive(Debug, Clone, PartialEq)]
pub struct LeptosTauriAppState {
    pub app_config: RwSignal<AppConfig>,
    pub logged_in: RwSignal<bool>,
}

impl Default for LeptosTauriAppState {
    fn default() -> Self {
        let app_config = create_rw_signal(AppConfig::default());
        app_config.update(|c| c.mode = Some(CommandsEnum::ClientNode));
        let logged_in = create_rw_signal(false);
        Self {
            app_config,
            logged_in,
        }
    }
}
