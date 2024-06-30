#[derive(Debug, Clone, Copy, Default)]
pub enum PageRoutes {
    #[default]
    Home,
    Settings,
    #[allow(dead_code)]
    Dashboard,
    Apps,
    OreMiner,
    ConfigViewer,
    Register,
    Login,
}

impl PageRoutes {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Settings => "/settings",
            Self::Dashboard => "/dashboard",
            Self::Apps => "/apps",
            Self::OreMiner => "/ore_miner",
            Self::ConfigViewer => "/config_viewer",
            Self::Register => "/register",
            Self::Login => "/login",
        }
    }
}
