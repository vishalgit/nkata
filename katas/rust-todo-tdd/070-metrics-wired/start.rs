use app::{App, shell, state::AppState};
use axum::{Router, extract::State, http::StatusCode, routing::get};
use leptos::prelude::provide_context;
use leptos_axum::{LeptosRoutes, generate_route_list};
use sqlx::PgPool;

async fn healthz(State(pool): State<PgPool>) -> (StatusCode, &'static str) {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(e) => {
            tracing::warn!(error = %e, "health check failed");
            (StatusCode::SERVICE_UNAVAILABLE, "database unavailable")
        }
    }
}

pub fn router(state: AppState) -> Router {
    let routes = generate_route_list(App);

    Router::new()
        .route("/healthz", get(healthz))
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
