use std::sync::Arc;

use crate::routes;
use crate::startup::application::AppState;
use axum::routing::get;
use axum::Router;
use block_mesh_common::routes_enum::RoutesEnum;

pub fn get_static_auth_router() -> Router<Arc<AppState>> {
    let auth_router = Router::new()
        .route(
            RoutesEnum::Static_Auth_Logout.to_string().as_str(),
            get(routes::logout::get::handler),
        )
        .route(
            RoutesEnum::Static_Auth_Dashboard.to_string().as_str(),
            get(routes::dashboard::get::handler).post(routes::dashboard::post::handler),
        );
    auth_router
}
