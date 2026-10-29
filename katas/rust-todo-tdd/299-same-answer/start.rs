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

#[sqlx::test(migrations = "../../migrations")]
async fn an_email_can_only_register_once(pool: PgPool) {
    let app = TestApp::new(pool);
    let first = app
        .post(
            "/api/auth/register",
            &[("email", "alice@example.com"), ("password", PASSWORD)],
            None,
        )
        .await;
    assert_eq!(first.status, StatusCode::OK);
    let res = app
        .post(
            "/api/auth/register",
            &[("email", "alice@example.com"), ("password", PASSWORD)],
            None,
        )
        .await;
    assert_eq!(res.status, StatusCode::CONFLICT);
}

#[sqlx::test(migrations = "../../migrations")]
async fn me_returns_the_user_behind_the_cookie(pool: PgPool) {
    let app = TestApp::new(pool);
    let cookie = app.register("alice@example.com").await;

    let me = app.post("/api/auth/me", &[], Some(&cookie)).await;
    assert_eq!(
        me.json::<Option<UserView>>(),
        Some(UserView {
            email: "alice@example.com".into(),
            is_admin: false
        })
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn without_a_cookie_nobody_is_logged_in(pool: PgPool) {
    let app = TestApp::new(pool);
    let me = app.post("/api/auth/me", &[], None).await;
    assert_eq!(me.json::<Option<UserView>>(), None);
}

#[sqlx::test(migrations = "../../migrations")]
async fn logging_in_with_the_right_password_starts_a_session(pool: PgPool) {
    let app = TestApp::new(pool);
    app.register("alice@example.com").await;

    let res = app
        .post(
            "/api/auth/login",
            &[("email", "alice@example.com"), ("password", PASSWORD)],
            None,
        )
        .await;
    assert_eq!(res.status, StatusCode::OK, "{}", res.body);
    assert!(res.session_cookie().is_some());
}
