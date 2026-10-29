mod common;

use common::{TestApp, scrape_metrics};

#[tokio::test]
async fn requests_are_counted_on_the_metrics_endpoint() {
    let app = TestApp::without_database();
    app.get("/healthz", None).await;

    let metrics = scrape_metrics().await;
    assert!(metrics.contains("http_requests_total"), "{metrics}");
    assert!(metrics.contains(r#"path="/healthz""#), "{metrics}");
}
