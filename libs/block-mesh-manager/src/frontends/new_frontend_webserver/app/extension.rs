use leptos::*;
use tailwind_fuse::*;

#[derive(Copy, Clone)]
pub enum CardMode {
    Login,
    Register,
    LoggedIn,
}

#[component]
pub fn ExtensionCard(mode: CardMode) -> impl IntoView {
    let content = match mode {
        
    }
}
