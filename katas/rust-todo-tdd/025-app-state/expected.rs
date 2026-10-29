use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use sqlx::PgPool;

/// Everything a request handler may need. Cloned per request (cheap: Arc inside).
#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
}
