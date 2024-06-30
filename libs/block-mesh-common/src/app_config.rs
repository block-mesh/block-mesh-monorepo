use crate::cli::{
    ClientNodeMode, ClientNodeOptions, Commands, CommandsEnum, ProxyEndpointNodeOptions,
    ProxyMasterNodeOptions,
};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct AppConfig {
    pub auto_start_blockmesh: Option<TaskStatus>,
    pub email: Option<String>,
    pub api_token: Option<String>,
    pub keypair_path: Option<String>,
    pub proxy_master_node_owner: Option<Pubkey>,
    pub program_id: Option<Pubkey>,
    pub proxy_override: Option<String>,
    pub proxy_port: Option<u16>,
    pub client_port: Option<u16>,
    pub mode: Option<CommandsEnum>,
    pub gui: Option<bool>,
    pub minimized: Option<bool>,
    pub config_path: Option<String>,
    pub task_status: Option<TaskStatus>,
    pub device_id: Option<Uuid>,
    pub ore_status: Option<TaskStatus>,
    pub ore_rpc: Option<String>,
    pub ore_threads: Option<u16>,
    pub ore_keypair: Option<String>,
    pub ore_priority_fee: Option<u64>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Copy)]
pub enum TaskStatus {
    Running,
    #[default]
    Off,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Running => write!(f, "Running"),
            TaskStatus::Off => write!(f, "Off"),
        }
    }
}

impl From<String> for TaskStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Running" => TaskStatus::Running,
            _ => TaskStatus::Off,
        }
    }
}

impl AppConfig {
    pub fn merge(&mut self, config: Self) {
        self.email = self.email.clone().or(config.email);
        self.api_token = self.api_token.clone().or(config.api_token);
        self.auto_start_blockmesh = self.auto_start_blockmesh.or(config.auto_start_blockmesh);
        self.keypair_path = self.keypair_path.clone().or(config.keypair_path);
        self.proxy_master_node_owner = self
            .proxy_master_node_owner
            .or(config.proxy_master_node_owner);
        self.program_id = self.program_id.or(config.program_id);
        self.proxy_override = self.proxy_override.clone().or(config.proxy_override);
        self.proxy_port = self.proxy_port.or(config.proxy_port);
        self.client_port = self.client_port.or(config.client_port);
        self.mode = self.mode.or(config.mode);
        self.gui = self.gui.or(config.gui);
        self.config_path = self.config_path.clone().or(config.config_path);
        self.device_id = self.device_id.or(config.device_id);
        self.ore_status = self.ore_status.or(config.ore_status);
        self.ore_rpc = self.ore_rpc.clone().or(config.ore_rpc);
        self.ore_threads = self.ore_threads.or(config.ore_threads);
        self.ore_keypair = self.ore_keypair.clone().or(config.ore_keypair);
        self.ore_priority_fee = self.ore_priority_fee.or(config.ore_priority_fee);
    }

    pub async fn validate_keypair(&self) -> anyhow::Result<()> {
        let path = env::current_dir()?;
        match &self.keypair_path {
            Some(keypair_path) => {
                let _ = solana_sdk::signature::read_keypair_file(keypair_path).map_err(|e| {
                    anyhow::anyhow!(
                        "Error reading keypair file, cwd: '{:?}', path: '{}' , error: {}",
                        path,
                        keypair_path,
                        e.to_string()
                    )
                })?;
                Ok(())
            }
            None => Err(anyhow::anyhow!("Keypair path not set")),
        }
    }
}

impl From<AppConfig> for Commands {
    fn from(app_config: AppConfig) -> Self {
        match app_config.mode {
            Some(CommandsEnum::ProxyMaster) => Commands::ProxyMaster(ProxyMasterNodeOptions {
                keypair_path: app_config.keypair_path.unwrap_or_default(),
                program_id: app_config.program_id.unwrap_or_default(),
                proxy_port: app_config.proxy_port.unwrap_or(5000),
                client_port: app_config.client_port.unwrap_or(4000),
                gui: app_config.gui.unwrap_or_default(),
            }),
            Some(CommandsEnum::ProxyEndpoint) => {
                Commands::ProxyEndpoint(ProxyEndpointNodeOptions {
                    keypair_path: app_config.keypair_path.unwrap_or_default(),
                    proxy_master_node_owner: app_config.proxy_master_node_owner,
                    program_id: app_config.program_id.unwrap_or_default(),
                    proxy_override: app_config.proxy_override,
                    gui: app_config.gui.unwrap_or_default(),
                })
            }
            None | Some(CommandsEnum::ClientNode) => Commands::ClientNode(ClientNodeOptions {
                keypair_path: app_config.keypair_path.unwrap_or_default(),
                proxy_master_node_owner: app_config.proxy_master_node_owner,
                program_id: app_config.program_id.unwrap_or_default(),
                target: "https://ifconfig.me/all.json".to_string(),
                proxy_override: app_config.proxy_override,
                mode: ClientNodeMode::Proxy,
                proxy_port: app_config.proxy_port.unwrap_or(8100),
                gui: app_config.gui.unwrap_or_default(),
            }),
        }
    }
}

impl From<Commands> for AppConfig {
    fn from(commands: Commands) -> Self {
        match commands {
            Commands::ClientNode(options) => AppConfig {
                email: None,
                api_token: None,
                auto_start_blockmesh: None,
                keypair_path: Some(options.keypair_path),
                proxy_master_node_owner: options.proxy_master_node_owner,
                program_id: Some(options.program_id),
                proxy_override: options.proxy_override,
                proxy_port: Some(options.proxy_port),
                client_port: None,
                mode: Some(CommandsEnum::ClientNode),
                gui: Some(options.gui),
                minimized: None,
                config_path: None,
                task_status: None,
                device_id: None,
                ore_status: None,
                ore_rpc: None,
                ore_threads: None,
                ore_keypair: None,
                ore_priority_fee: None,
            },
            Commands::ProxyMaster(options) => AppConfig {
                email: None,
                api_token: None,
                auto_start_blockmesh: None,
                keypair_path: Some(options.keypair_path),
                proxy_master_node_owner: None,
                program_id: Some(options.program_id),
                proxy_override: None,
                proxy_port: Some(options.proxy_port),
                client_port: Some(options.client_port),
                mode: Some(CommandsEnum::ProxyMaster),
                gui: Some(options.gui),
                minimized: None,
                config_path: None,
                task_status: None,
                device_id: None,
                ore_status: None,
                ore_rpc: None,
                ore_threads: None,
                ore_keypair: None,
                ore_priority_fee: None,
            },
            Commands::ProxyEndpoint(options) => AppConfig {
                email: None,
                api_token: None,
                auto_start_blockmesh: None,
                keypair_path: Some(options.keypair_path),
                proxy_master_node_owner: options.proxy_master_node_owner,
                program_id: Some(options.program_id),
                proxy_override: options.proxy_override,
                proxy_port: None,
                client_port: None,
                mode: Some(CommandsEnum::ProxyEndpoint),
                gui: Some(options.gui),
                minimized: None,
                config_path: None,
                task_status: None,
                device_id: None,
                ore_status: None,
                ore_rpc: None,
                ore_threads: None,
                ore_keypair: None,
                ore_priority_fee: None,
            },
        }
    }
}
