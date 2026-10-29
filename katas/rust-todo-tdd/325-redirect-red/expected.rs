mod common;

use axum::http::StatusCode;
use common::TestApp;
use sqlx::PgPool;

#[sqlx::test(migrations = "../../migrations")]
async fn anonymous_visitors_are_sent_to_the_login_page(pool: PgPool) {
    let app = TestApp::new(pool);
    let res = app.get("/", None).await;
    assert!(res.status.is_redirection(), "status was {}", res.status);
    assert_eq!(res.headers["location"], "/login");
}
