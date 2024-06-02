use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;

#[tracing::instrument(name = "register")]
pub async fn handler(
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Redirect> {
    return match auth.user {
        Some(_) => Err(Redirect::to("/dashboard")),
        None => Ok(RegisterTemplate),
    };
}
