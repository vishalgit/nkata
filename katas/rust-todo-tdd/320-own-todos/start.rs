mod common;

use app::error::AppError;
use app::features::todos::{TodoPage, TodoView};
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

#[sqlx::test(migrations = "../../migrations")]
async fn an_empty_title_is_unprocessable(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let res = app
        .post("/api/todos/create", &[("title", "   ")], Some(&alice))
        .await;
    assert_eq!(res.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrations = "../../migrations")]
async fn a_created_todo_is_returned_and_listed(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;

    let todo: TodoView = app
        .post(
            "/api/todos/create",
            &[("title", "  Buy milk ")],
            Some(&alice),
        )
        .await
        .json();
    assert_eq!(todo.title, "Buy milk");
    assert!(!todo.completed);

    let page: TodoPage = app.post("/api/todos/list", &[], Some(&alice)).await.json();
    assert_eq!(page.items, vec![todo]);
    assert_eq!((page.page, page.total_pages, page.total), (1, 1, 1));
}
