#[derive(Debug, Clone, Copy, Default)]
pub enum PageRoutes {
    #[default]
    Home,
    Settings,
    Dashboard,
}

impl PageRoutes {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Settings => "/settings",
            Self::Dashboard => "/dashboard",
        }
    }
}
