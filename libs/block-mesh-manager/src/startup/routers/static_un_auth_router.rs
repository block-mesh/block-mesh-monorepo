use crate::routes;
use crate::startup::application::AppState;
use axum::routing::{get, post};
use axum::Router;
use block_mesh_common::routes_enum::RoutesEnum;
use std::sync::Arc;

pub fn get_static_un_auth_router() -> Router<Arc<AppState>> {
    let un_auth_router = Router::new()
        .route(
            RoutesEnum::Static_UnAuth_AuthStatus.to_string().as_str(),
            get(routes::health_check::auth_status::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_RpcDashboard.to_string().as_str(),
            get(routes::rpc::rpc_dashboard::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_RpcApi.to_string().as_str(),
            get(routes::rpc::rpc_api::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_Notification.to_string().as_str(),
            get(routes::notification::notification_page::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_EmailConfirm.to_string().as_str(),
            get(routes::emails::email_confirm::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_ResetPassword.to_string().as_str(),
            get(routes::password::reset_password_form::handler)
                .post(routes::password::reset_password_post::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_ResendConfirmationEmail
                .to_string()
                .as_str(),
            get(routes::emails::resend_confirm_email_form::handler)
                .post(routes::emails::resend_confirm_email_post::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_NewPassword.to_string().as_str(),
            get(routes::password::new_password_form::handler)
                .post(routes::password::new_password_post::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
            get(routes::login::login_form::handler).post(routes::login::login_post::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_Error.to_string().as_str(),
            get(routes::error::error_page::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
            get(routes::login::login_form::handler).post(routes::login::login_post::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_RegisterApi.to_string().as_str(),
            post(routes::register::register_api::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
            get(routes::register::register_form::handler)
                .post(routes::register::register_post::handler),
        )
        .route(
            RoutesEnum::Static_UnAuth_Twitter_Callback
                .to_string()
                .as_str(),
            get(routes::twitter::callback::callback),
        )
        .route(
            RoutesEnum::Static_UnAuth_HealthCheck.to_string().as_str(),
            get(routes::health_check::get::handler),
        );
    un_auth_router
}
