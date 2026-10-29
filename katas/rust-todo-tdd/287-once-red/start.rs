mod common;

use app::error::AppError;
use app::features::auth::UserView;
use axum::http::StatusCode;
use common::{PASSWORD, TestApp};
use sqlx::PgPool;

#[sqlx::test(migrations = "../../migrations")]
async fn registering_returns_the_user_and_starts_a_session(pool: PgPool) {
    let app = TestApp::new(pool);
    let res = app
        .post(
            "/api/auth/register",
            &[("email", "Alice@Example.com"), ("password", PASSWORD)],
            None,
        )
        .await;

    assert_eq!(res.status, StatusCode::OK, "{}", res.body);
    assert_eq!(
        res.json::<UserView>(),
        UserView {
            email: "alice@example.com".into(),
            is_admin: false
        }
    );
    assert!(res.session_cookie().is_some());
}

#[sqlx::test(migrations = "../../migrations")]
async fn an_invalid_registration_lists_every_problem(pool: PgPool) {
    let app = TestApp::new(pool);
    let res = app
        .post(
            "/api/auth/register",
            &[("email", "nope"), ("password", "short")],
            None,
        )
        .await;

    assert_eq!(res.status, StatusCode::UNPROCESSABLE_ENTITY);
    match res.json::<AppError>() {
        AppError::Validation(problems) => assert_eq!(problems.len(), 2, "{problems:?}"),
        other => panic!("expected Validation, got {other:?}"),
    }
}
