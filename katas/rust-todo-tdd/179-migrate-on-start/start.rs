use app::state::AppState;
use leptos::config::get_configuration;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let filter = EnvFilter::from_default_env();
    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .init();
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }

    // cargo-leptos (dev) or LEPTOS_* env vars (production) provide the settings
    let conf = get_configuration(None).expect("leptos configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // connect_lazy: the server starts even if Postgres is still booting; /healthz tells the truth
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&database_url)
        .expect("valid DATABASE_URL");

    let app = server::router(AppState {
        leptos_options,
        pool,
    });

    let metrics_addr = std::env::var("METRICS_ADDR").unwrap_or_else(|_| "0.0.0.0:9000".into());
    let metrics_listener = tokio::net::TcpListener::bind(&metrics_addr)
        .await
        .expect("bind metrics");
    tokio::spawn(async move {
        axum::serve(metrics_listener, server::metrics_router())
            .await
            .expect("metrics server");
    });

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("server error");
}
