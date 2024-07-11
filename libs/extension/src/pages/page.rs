#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
        }
    }
}
