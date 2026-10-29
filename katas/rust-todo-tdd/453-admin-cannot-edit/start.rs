mod common;

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
