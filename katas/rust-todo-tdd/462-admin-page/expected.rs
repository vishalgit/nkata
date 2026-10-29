use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
use crate::features::todos::delete::DeleteTodo;
use crate::ui::ErrorMessage;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminTodoView {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub owner_email: String,
}

#[server(endpoint = "admin/todos")]
pub async fn admin_todos() -> Result<Vec<AdminTodoView>, AppError> {
    shell::handle().await.map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use domain::identity::{Action, authorize};

    use super::*;
    use crate::effects;

    pub async fn handle() -> Result<Vec<AdminTodoView>, AppError> {
        let user = effects::require_user().await?;
        authorize(&user.actor, Action::ViewAllTodos)?;
        let state = effects::state()?;

        let rows = sqlx::query!(
            r#"SELECT t.id, t.title, t.completed_at IS NOT NULL AS "completed!", u.email
               FROM todos t JOIN users u ON u.id = t.owner_id
               ORDER BY u.email, t.created_at DESC"#
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "admin todo query failed");
            AppError::Unexpected
        })?;

        Ok(rows
            .into_iter()
            .map(|r| AdminTodoView {
                id: r.id,
                title: r.title,
                completed: r.completed,
                owner_email: r.email,
            })
            .collect())
    }
}

#[component]
pub fn AdminTodoTable(todos: Vec<AdminTodoView>) -> impl IntoView {
    let delete = expect_context::<ServerAction<DeleteTodo>>();
    view! {
        <table class="data-table">
            <thead>
                <tr><th>"Owner"</th><th>"Title"</th><th>"Status"</th><th></th></tr>
            </thead>
            <tbody>
                {todos.into_iter().map(|t| view! {
                    <tr data-testid="admin-row">
                        <td>{t.owner_email}</td>
                        <td>{t.title}</td>
                        <td>{if t.completed { "done" } else { "open" }}</td>
                        <td>
                            <ActionForm action=delete>
                                <input type="hidden" name="id" value=t.id.to_string()/>
                                <button class="button button--small button--danger" type="submit">"Delete"</button>
                            </ActionForm>
                        </td>
                    </tr>
                }).collect_view()}
            </tbody>
        </table>
    }
}

#[component]
pub fn AdminPage() -> impl IntoView {
    let delete = ServerAction::<DeleteTodo>::new();
    provide_context(delete);
    let todos = Resource::new(move || delete.version().get(), |_| admin_todos());
    let error = Signal::derive(move || delete.value().get().and_then(Result::err));

    view! {
        <section class="admin">
            <h1>"All todos"</h1>
            <ErrorMessage error/>
            <Suspense fallback=|| view! { <p class="muted">"Loading…"</p> }>
                {move || todos.get().map(|result| match result {
                    Ok(list) => view! { <AdminTodoTable todos=list/> }.into_any(),
                    Err(AppError::Forbidden) => view! {
                        <p class="alert alert--error">"You are not allowed to see this page."</p>
                    }.into_any(),
                    Err(e) => view! { <p class="alert alert--error">{e.to_string()}</p> }.into_any(),
                })}
            </Suspense>
        </section>
    }
}

#[cfg(all(test, feature = "ssr"))]
mod ui_tests {
    use super::*;
    use crate::ui::testing::render;

    #[test]
    fn each_row_shows_the_owner_and_a_delete_button() {
        let rows = vec![AdminTodoView {
            id: Uuid::nil(),
            title: "Alice's".into(),
            completed: true,
            owner_email: "alice@example.com".into(),
        }];
        let html = render(move || {
            provide_context(ServerAction::<DeleteTodo>::new());
            view! { <AdminTodoTable todos=rows/> }
        });
        assert!(html.contains("alice@example.com"), "{html}");
        assert!(html.contains(">done<"));
        assert!(html.contains(">Delete<"));
    }
}
