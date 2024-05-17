use block_mesh_common::app_config::AppConfig;
use block_mesh_common::cli::CommandsEnum;
use leptos::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LeptosTauriAppState {
    pub app_config: RwSignal<AppConfig>,
}

impl Default for LeptosTauriAppState {
    fn default() -> Self {
        let app_config = create_rw_signal(AppConfig::default());
        app_config.update(|c| c.mode = Some(CommandsEnum::ClientNode));
        Self { app_config }
    }
}
