use std::rc::Rc;
use leptos::*;
use leptos_use::{use_toggle, UseToggleReturn};

#[derive(Clone)]
pub struct ReloadContext {
    toggle: Rc<dyn Fn()>,
    pub value: Signal<bool>,
}

impl ReloadContext {
    pub fn trigger_reload(&self) {
        (self.toggle)();
    }
}

impl Default for ReloadContext {
    fn default() -> Self {
        let UseToggleReturn { value, toggle, .. } = use_toggle(false);
        Self { toggle: Rc::new(toggle), value }
    }
}