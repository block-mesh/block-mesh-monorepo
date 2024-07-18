use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

use leptos::*;
use leptos_dom::tracing;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, Copy)]
pub struct NotificationContext {
    pub success: RwSignal<Option<String>>,
    pub error: RwSignal<Option<String>>,
}

impl Debug for NotificationContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotificationState")
            .field("success", &self.success.get_untracked())
            .field("error", &self.error.get_untracked())
            .finish()
    }
}

impl NotificationContext {
    #[tracing::instrument(name = "NotificationState::set_success")]
    pub fn set_success<T>(&self, success: T)
    where
        T: Display + Clone + Into<String> + Debug,
    {
        let signal = self.success;
        let success = Option::from(success.clone().to_string());
        signal.update(|v| *v = success);
        set_timeout(
            move || {
                signal.update(|v| *v = None);
            },
            Duration::from_millis(3500),
        );
    }

    #[tracing::instrument(name = "NotificationState::set_error")]
    pub fn set_error<T>(&self, error: T)
    where
        T: Display + Clone + Into<String> + Debug,
    {
        let signal = self.error;
        let error = Option::from(error.clone().to_string());
        signal.update(|v| *v = error);
        set_timeout(
            move || {
                signal.update(|v| *v = None);
            },
            Duration::from_millis(3500),
        );
    }
}
