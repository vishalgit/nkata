use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

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
