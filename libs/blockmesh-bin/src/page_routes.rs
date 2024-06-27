#[derive(Debug, Clone, Copy, Default)]
pub enum PageRoutes {
    #[default]
    Home,
    Settings,
    Dashboard,
    Apps,
    OreMiner,
}

impl PageRoutes {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Settings => "/settings",
            Self::Dashboard => "/dashboard",
            Self::Apps => "/apps",
            Self::OreMiner => "/ore_miner",
        }
    }
}
