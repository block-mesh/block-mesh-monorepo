use leptos::ev::resize;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct SizeContext {
    pub width: RwSignal<f64>,
    pub height: RwSignal<f64>,
}

impl Debug for SizeContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthContext")
            .field("width", &self.width.get_untracked())
            .field("height", &self.height.get_untracked())
            .finish()
    }
}

impl Default for SizeContext {
    fn default() -> Self {
        let width = create_rw_signal(0.0);
        let height = create_rw_signal(0.0);

        create_effect(move |_| {
            width.set(window().inner_width().unwrap().as_f64().unwrap());
            height.set(window().inner_height().unwrap().as_f64().unwrap());

            window_event_listener(resize, move |_| {
                width.set(window().inner_width().unwrap().as_f64().unwrap());
                height.set(window().inner_height().unwrap().as_f64().unwrap());
            });
        });

        Self { width, height }
    }
}
