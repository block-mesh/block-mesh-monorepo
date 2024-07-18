use block_mesh_common::chrome_storage::AuthStatus;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Formatter};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Default, Copy)]
pub struct AuthContext {
    pub email: RwSignal<String>,
    pub api_token: RwSignal<Uuid>,
    pub device_id: RwSignal<Uuid>,
    pub blockmesh_url: RwSignal<String>,
    pub invite_code: RwSignal<String>,
    pub status: RwSignal<AuthStatus>,
}

impl Debug for AuthContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthContext")
            .field("email", &self.email.get_untracked())
            .field("device_id", &self.device_id.get_untracked())
            .field("api_token", &"********")
            .field("blockmesh_url", &self.blockmesh_url.get_untracked())
            .field("invite_code", &self.invite_code.get_untracked())
            .field("status", &self.status.get_untracked())
            .finish()
    }
}
