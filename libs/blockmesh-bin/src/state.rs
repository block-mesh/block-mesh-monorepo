use block_mesh_common::cli::{CliArgs, ClientNodeOptions, Commands};
use leptos::{create_rw_signal, RwSignal};

#[derive(Debug, Clone, PartialEq)]
pub struct LeptosTauriAppState {
    pub cli_args: RwSignal<CliArgs>,
}

impl Default for LeptosTauriAppState {
    fn default() -> Self {
        Self {
            cli_args: create_rw_signal(CliArgs {
                minimized: false,
                command: Some(Commands::ClientNode(ClientNodeOptions::default())),
            }),
        }
    }
}
