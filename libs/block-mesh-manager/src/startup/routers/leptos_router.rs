use crate::frontends::app::App;
use axum::routing::post;
use axum::Router;
use leptos::leptos_config::get_config_from_env;
use leptos_axum::{generate_route_list, LeptosRoutes};

pub fn get_leptos_router() -> Router<()> {
    let leptos_config = get_config_from_env().unwrap();
    let leptos_options = leptos_config.leptos_options;
    let routes = generate_route_list(App);
    let leptos_router: Router<()> = Router::new()
        .route("/leptos_api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .with_state(leptos_options);
    leptos_router
}
