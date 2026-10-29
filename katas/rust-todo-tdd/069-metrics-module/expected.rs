use std::{sync::OnceLock, time::Instant};

use axum::{
    Router,
    extract::{MatchedPath, Request},
    middleware::Next,
    response::Response,
    routing::get,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

static HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

/// Installs the global recorder exactly once (also once per test binary).
pub fn handle() -> &'static PrometheusHandle {
    HANDLE.get_or_init(|| {
        PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full("http_request_duration_seconds".into()),
                &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0],
            )
            .expect("valid buckets")
            .install_recorder()
            .expect("metrics recorder installs once")
    })
}

/// Served on its own port, never through the ingress.
pub fn metrics_router() -> Router {
    Router::new().route("/metrics", get(|| async { handle().render() }))
}

/// Middleware: one counter, one latency histogram and one structured log line per request.
pub async fn track(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().to_string();
    // the route pattern ("/api/todos/create"), not the raw URL: keeps label cardinality low
    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_owned())
        .unwrap_or_else(|| "unmatched".into());

    let response = next.run(req).await;

    let status = response.status().as_u16().to_string();
    let seconds = start.elapsed().as_secs_f64();
    metrics::counter!("http_requests_total",
        "method" => method.clone(), "path" => path.clone(), "status" => status.clone())
    .increment(1);
    metrics::histogram!("http_request_duration_seconds",
        "method" => method.clone(), "path" => path.clone())
    .record(seconds);
    tracing::info!(%method, %path, %status, latency_ms = seconds * 1000.0, "request");
    response
}
