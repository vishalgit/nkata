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

#[sqlx::test(migrations = "../../migrations")]
async fn the_login_page_renders_a_working_form(pool: PgPool) {
    let app = TestApp::new(pool);
    let res = app.get("/login", None).await;
    assert_eq!(res.status, StatusCode::OK);
    assert!(res.body.contains(r#"name="email""#));
    assert!(res.body.contains(r#"name="password""#));
    assert!(
        res.body.contains(r#"action="/api/auth/login""#),
        "form posts to the server function"
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_header_shows_who_is_logged_in(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let res = app.get("/", Some(&alice)).await;
    assert_eq!(res.status, StatusCode::OK);
    assert!(
        res.body.contains(r#"site-header__user">alice@example.com"#),
        "{}",
        res.body
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_register_page_renders_a_working_form(pool: PgPool) {
    let app = TestApp::new(pool);
    let res = app.get("/register", None).await;
    assert_eq!(res.status, StatusCode::OK);
    assert!(res.body.contains(r#"action="/api/auth/register""#));
}
