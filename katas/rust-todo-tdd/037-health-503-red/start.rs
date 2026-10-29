mod common;

use axum::http::StatusCode;
use common::TestApp;

#[tokio::test]
async fn the_home_page_renders() {
    let app = TestApp::without_database();
    let res = app.get("/", None).await;
    assert_eq!(res.status, StatusCode::OK);
    assert!(res.body.contains("<h1>Todo</h1>"), "{}", res.body);
}

#[tokio::test]
async fn health_is_ok_when_the_database_answers() {
    let app = TestApp::with_database().await;
    let res = app.get("/healthz", None).await;
    assert_eq!(res.status, StatusCode::OK);
    assert_eq!(res.body, "ok");
}
