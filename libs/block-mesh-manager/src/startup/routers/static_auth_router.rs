use std::sync::Arc;

use crate::routes;
use crate::startup::application::AppState;
use axum::routing::{get, post};
use axum::Router;
use block_mesh_common::routes_enum::RoutesEnum;

pub fn get_static_auth_router() -> Router<Arc<AppState>> {
    let auth_router = Router::new()
        .route(
            RoutesEnum::Static_Auth_Proof_Of_Human.to_string().as_str(),
            get(routes::proof_of_human::proof_of_human_get::handler)
                .post(routes::proof_of_human::proof_of_human_post::handler),
        )
        .route(
            RoutesEnum::Static_Auth_Logout.to_string().as_str(),
            get(routes::logout::get::handler),
        )
        .route(
            RoutesEnum::Static_Auth_Twitter_Login.to_string().as_str(),
            get(routes::twitter::login::login),
        )
        .route(
            RoutesEnum::Static_Auth_Edit_Invite.to_string().as_str(),
            post(routes::invite_codes::edit_invite_code_post::handler),
        )
        .route(
            RoutesEnum::Static_Auth_ResendConfirmationEmail
                .to_string()
                .as_str(),
            get(routes::emails::resend_confirm_email_form::handler)
                .post(routes::emails::resend_confirm_email_post::handler),
        )
        .route(
            RoutesEnum::Static_Auth_Call_To_Action.to_string().as_str(),
            post(routes::call_to_action::post::handler),
        )
        .route(
            RoutesEnum::Static_Auth_Daily_Leaderboard
                .to_string()
                .as_str(),
            post(routes::leaderboard::daily_leaderboard::handler),
        )
        .route(
            RoutesEnum::Static_Auth_Dashboard.to_string().as_str(),
            post(routes::dashboard::post::handler),
        );
    auth_router
}
