use app::{App, shell, state::AppState};
use axum::Router;
use leptos::prelude::provide_context;
use leptos_axum::{LeptosRoutes, generate_route_list};

pub fn router(state: AppState) -> Router {
    let routes = generate_route_list(App);

    Router::new()
        // Page routes AND every #[server] function, each with AppState in context
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let state = state.clone();
                move || provide_context(state.clone())
            },
            {
                let options = state.leptos_options.clone();
                move || shell(options.clone())
            },
        )
        // Static files from target/site, or the 404 page
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(state)
}
