use leptos::prelude::*;

use super::{TodoPage, TodoView};
use crate::error::AppError;

#[server(endpoint = "todos/list")]
pub async fn list_todos(
    filter: Option<String>,
    q: Option<String>,
    page: Option<u32>,
) -> Result<TodoPage, AppError> {
    shell::handle(filter, q, page)
        .await
        .map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use domain::todos::TodoQuery;

    use super::*;
    use crate::effects;
    use crate::features::todos::store;

    pub async fn handle(
        filter: Option<String>,
        q: Option<String>,
        page: Option<u32>,
    ) -> Result<TodoPage, AppError> {
        let user = effects::require_user().await?;
        let state = effects::state()?;
        // pure: three raw strings become one valid query
        let query = TodoQuery::parse(filter.as_deref(), q.as_deref(), page)?;
        let paged = store::search_for_owner(&state.pool, user.actor.id, &query).await?;
        Ok(TodoPage::from(paged))
    }
}

#[component]
pub fn TodoItem(todo: TodoView) -> impl IntoView {
    view! {
        <li class="todo-item" data-testid="todo-item">
            <span class="todo-item__title">{todo.title}</span>
        </li>
    }
}

#[cfg(all(test, feature = "ssr"))]
mod ui_tests {
    use super::*;
    use crate::ui::testing::render;
    use uuid::Uuid;

    fn todo(title: &str, completed: bool) -> TodoView {
        TodoView {
            id: Uuid::new_v4(),
            title: title.into(),
            completed,
        }
    }

    #[test]
    fn an_item_shows_its_title_with_bem_classes() {
        let html = render(|| view! { <TodoItem todo=todo("Buy milk", false)/> });
        assert!(html.contains(r#"class="todo-item""#), "{html}");
        assert!(html.contains("Buy milk"));
    }

    #[test]
    fn a_completed_item_gets_the_modifier_class() {
        let html = render(|| view! { <TodoItem todo=todo("Buy milk", true)/> });
        assert!(html.contains("todo-item todo-item--completed"), "{html}");
    }
}
