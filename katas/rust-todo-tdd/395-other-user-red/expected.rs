mod common;

use app::error::AppError;
use app::features::todos::{TodoPage, TodoView};
use axum::http::StatusCode;
use common::TestApp;
use sqlx::PgPool;

async fn create(app: &TestApp, cookie: &str, title: &str) -> TodoView {
    app.post("/api/todos/create", &[("title", title)], Some(cookie))
        .await
        .json()
}

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

#[sqlx::test(migrations = "../../migrations")]
async fn users_only_see_their_own_todos(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let bob = app.register("bob@example.com").await;
    app.post("/api/todos/create", &[("title", "Alice's")], Some(&alice))
        .await;

    let page: TodoPage = app.post("/api/todos/list", &[], Some(&bob)).await.json();
    assert!(page.items.is_empty());
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_list_is_paged(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    for i in 0..12 {
        create(&app, &alice, &format!("Task {i:02}")).await;
    }

    let page: TodoPage = app
        .post("/api/todos/list", &[("page", "2")], Some(&alice))
        .await
        .json();
    assert_eq!(
        (page.items.len(), page.page, page.total_pages, page.total),
        (2, 2, 2, 12)
    );
}

#[sqlx::test(migrations = "../../migrations")]
async fn search_and_filter_work_together_over_http(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    create(&app, &alice, "Buy milk").await;
    create(&app, &alice, "Walk the dog").await;

    let page: TodoPage = app
        .post(
            "/api/todos/list",
            &[("q", "MILK"), ("filter", "active")],
            Some(&alice),
        )
        .await
        .json();
    let titles: Vec<String> = page.items.into_iter().map(|t| t.title).collect();
    assert_eq!(titles, vec!["Buy milk"]);
}

#[sqlx::test(migrations = "../../migrations")]
async fn an_overlong_search_is_unprocessable(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let long = "x".repeat(101);
    let res = app
        .post("/api/todos/list", &[("q", long.as_str())], Some(&alice))
        .await;
    assert_eq!(res.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test(migrations = "../../migrations")]
async fn completing_a_todo_marks_it_done(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let id = create(&app, &alice, "Buy milk").await.id.to_string();

    let done = app
        .post("/api/todos/complete", &[("id", &id)], Some(&alice))
        .await;
    assert_eq!(done.status, StatusCode::OK, "{}", done.body);
    assert!(done.json::<TodoView>().completed);
}

#[sqlx::test(migrations = "../../migrations")]
async fn completing_twice_is_a_conflict(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let id = create(&app, &alice, "Buy milk").await.id.to_string();

    app.post("/api/todos/complete", &[("id", &id)], Some(&alice))
        .await;
    let again = app
        .post("/api/todos/complete", &[("id", &id)], Some(&alice))
        .await;
    assert_eq!(again.status, StatusCode::CONFLICT);
}

#[sqlx::test(migrations = "../../migrations")]
async fn another_users_todo_does_not_exist_for_you(pool: PgPool) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let bob = app.register("bob@example.com").await;
    let id = create(&app, &alice, "Alice's").await.id.to_string();

    let res = app
        .post("/api/todos/complete", &[("id", &id)], Some(&bob))
        .await;
    assert_eq!(res.status, StatusCode::NOT_FOUND);
}
