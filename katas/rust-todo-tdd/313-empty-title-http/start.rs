mod common;

use app::error::AppError;
use axum::http::StatusCode;
use common::TestApp;
use sqlx::PgPool;

#[sqlx::test(migrations = "../../migrations")]
async fn creating_a_todo_requires_a_login(pool: PgPool) {
    let app = TestApp::new(pool);
    let res = app
        .post("/api/todos/create", &[("title", "Buy milk")], None)
        .await;
    assert_eq!(res.status, StatusCode::UNAUTHORIZED);
    assert_eq!(res.json::<AppError>(), AppError::Unauthenticated);
}
