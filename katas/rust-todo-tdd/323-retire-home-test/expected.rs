mod common;

use axum::http::StatusCode;
use common::TestApp;

#[tokio::test]
async fn health_is_ok_when_the_database_answers() {
    let app = TestApp::with_database().await;
    let res = app.get("/healthz", None).await;
    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(res.body, "ok");
}

#[tokio::test]
async fn health_is_503_when_the_database_is_down() {
    let app = TestApp::without_database();
    let res = app.get("/healthz", None).await;
    assert_eq!(res.status, StatusCode::SERVICE_UNAVAILABLE);
}
