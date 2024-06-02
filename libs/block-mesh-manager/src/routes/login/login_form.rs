use askama::Template;
use askama_axum::IntoResponse;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;

use crate::middlewares::authentication::Backend;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[tracing::instrument(name = "login")]
pub async fn handler(
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Redirect> {
    return match auth.user {
        Some(_) => Err(Redirect::to("/dashboard")),
        None => Ok(LoginTemplate),
    };
}
