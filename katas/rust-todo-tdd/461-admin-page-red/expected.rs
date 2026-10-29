mod common;

use app::features::admin::AdminTodoView;
use app::features::todos::TodoView;
use axum::http::StatusCode;
use common::TestApp;
use sqlx::PgPool;

/// Alice owns one todo; admin@example.com is an admin.
async fn setup(pool: PgPool) -> (TestApp, String, String, String) {
    let app = TestApp::new(pool);
    let alice = app.register("alice@example.com").await;
    let admin = app.register("admin@example.com").await;
    app.make_admin("admin@example.com").await;
    let todo: TodoView = app
        .post("/api/todos/create", &[("title", "Alice's")], Some(&alice))
        .await
        .json();
    (app, alice, admin, todo.id.to_string())
}

#[sqlx::test(migrations = "../../migrations")]
async fn an_admin_can_delete_anyones_todo(pool: PgPool) {
    let (app, _alice, admin, id) = setup(pool).await;
    let res = app
        .post("/api/todos/delete", &[("id", &id)], Some(&admin))
        .await;
    assert_eq!(res.status, StatusCode::OK, "{}", res.body);
}

#[sqlx::test(migrations = "../../migrations")]
async fn an_admin_cannot_edit_someone_elses_todo(pool: PgPool) {
    let (app, _alice, admin, id) = setup(pool).await;
    let res = app
        .post(
            "/api/todos/rename",
            &[("id", &id), ("title", "Moderated")],
            Some(&admin),
        )
        .await;
    assert_eq!(res.status, StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "../../migrations")]
async fn only_admins_see_all_todos(pool: PgPool) {
    let (app, alice, admin, _id) = setup(pool).await;

    assert_eq!(
        app.post("/api/admin/todos", &[], Some(&alice)).await.status,
        StatusCode::FORBIDDEN
    );

    let all: Vec<AdminTodoView> = app.post("/api/admin/todos", &[], Some(&admin)).await.json();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].owner_email, "alice@example.com");
}

#[sqlx::test(migrations = "../../migrations")]
async fn the_admin_page_is_closed_to_ordinary_users(pool: PgPool) {
    let (app, alice, admin, _id) = setup(pool).await;
    assert!(
        app.get("/admin", Some(&alice))
            .await
            .body
            .contains("not allowed")
    );
    assert!(
        app.get("/admin", Some(&admin))
            .await
            .body
            .contains("alice@example.com")
    );
}
