use leptos::RwSignal;
use leptos::*;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct TauriState {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<String>,
}

impl Debug for TauriState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MobileState")
            .field("email", &self.email.get_untracked())
            .field("api_token", &"********")
            .finish()
    }
}
