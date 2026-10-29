use leptos::prelude::*;

use domain::todos::Filter;

use super::actions::use_todo_actions;
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

/// The URL for a list view. Defaults are left out, so links stay short.
pub fn list_href(filter: Filter, q: Option<&str>, page: u32) -> String {
    let mut params: Vec<(&str, String)> = Vec::new();
    if filter != Filter::All {
        params.push(("filter", filter.as_str().to_owned()));
    }
    if let Some(q) = q.filter(|q| !q.is_empty()) {
        params.push(("q", q.to_owned()));
    }
    if page > 1 {
        params.push(("page", page.to_string()));
    }
    if params.is_empty() {
        "/".into()
    } else {
        format!(
            "/?{}",
            serde_urlencoded::to_string(&params).expect("strings always encode")
        )
    }
}

#[component]
pub fn TodoItem(todo: TodoView) -> impl IntoView {
    let actions = use_todo_actions();
    let class = if todo.completed {
        "todo-item todo-item--completed"
    } else {
        "todo-item"
    };
    let id = todo.id.to_string();

    let toggle = if todo.completed {
        let id = id.clone();
        view! {
            <ActionForm action=actions.reopen>
                <input type="hidden" name="id" value=id/>
                <button class="button button--small button--ghost" type="submit">"Undo"</button>
            </ActionForm>
        }
        .into_any()
    } else {
        let id = id.clone();
        view! {
            <ActionForm action=actions.complete>
                <input type="hidden" name="id" value=id/>
                <button class="button button--small button--ghost" type="submit">"Done"</button>
            </ActionForm>
        }
        .into_any()
    };

    let rename_id = id.clone();
    view! {
        <li class=class data-testid="todo-item">
            {toggle}
            <span class="todo-item__title">{todo.title.clone()}</span>
            <div class="todo-item__actions">
                <details class="todo-item__edit">
                    <summary class="button button--small button--ghost">"Rename"</summary>
                    <ActionForm action=actions.rename attr:class="todo-item__rename">
                        <input type="hidden" name="id" value=rename_id/>
                        <input class="form__input" type="text" name="title" value=todo.title
                            required maxlength="120" aria-label="New title"/>
                        <button class="button button--small button--primary" type="submit">"Save"</button>
                    </ActionForm>
                </details>
                <ActionForm action=actions.delete>
                    <input type="hidden" name="id" value=id/>
                    <button class="button button--small button--danger" type="submit">"Delete"</button>
                </ActionForm>
            </div>
        </li>
    }
}

#[component]
pub fn TodoList(todos: Vec<TodoView>) -> impl IntoView {
    if todos.is_empty() {
        return view! { <p class="todo-list__empty">"Nothing to do. Add your first todo above."</p> }.into_any();
    }
    view! {
        <ul class="todo-list">
            {todos.into_iter().map(|todo| view! { <TodoItem todo/> }).collect_view()}
        </ul>
    }
    .into_any()
}

#[cfg(test)]
mod href_tests {
    use super::*;

    #[test]
    fn the_default_query_is_a_bare_slash() {
        assert_eq!(list_href(Filter::All, None, 1), "/");
    }

    #[test]
    fn every_part_is_kept_and_encoded() {
        assert_eq!(
            list_href(Filter::Active, Some("oat milk & honey"), 2),
            "/?filter=active&q=oat+milk+%26+honey&page=2"
        );
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

    use crate::features::todos::actions::TodoActions;

    fn render_item(todo: TodoView) -> String {
        render(move || {
            TodoActions::provide();
            view! { <TodoItem todo/> }
        })
    }

    #[test]
    fn an_item_shows_its_title_with_bem_classes() {
        let html = render_item(todo("Buy milk", false));
        assert!(html.contains(r#"class="todo-item""#), "{html}");
        assert!(html.contains("Buy milk"));
    }

    #[test]
    fn a_completed_item_gets_the_modifier_class() {
        let html = render_item(todo("Buy milk", true));
        assert!(html.contains("todo-item todo-item--completed"), "{html}");
    }

    #[test]
    fn an_empty_list_shows_a_friendly_hint() {
        let html = render(|| view! { <TodoList todos=vec![]/> });
        assert!(html.contains("todo-list__empty"), "{html}");
    }

    #[test]
    fn a_list_renders_one_item_per_todo() {
        let html = render(|| {
            TodoActions::provide();
            view! { <TodoList todos=vec![todo("A", false), todo("B", true)]/> }
        });
        assert_eq!(
            html.matches(r#"data-testid="todo-item""#).count(),
            2,
            "{html}"
        );
    }

    #[test]
    fn an_active_item_offers_done_and_a_completed_one_offers_undo() {
        assert!(render_item(todo("A", false)).contains(">Done<"));
        assert!(render_item(todo("B", true)).contains(">Undo<"));
    }

    #[test]
    fn every_action_form_posts_the_todo_id() {
        let t = todo("A", false);
        let html = render_item(t.clone());
        let hidden = format!(r#"name="id" value="{}""#, t.id);
        assert_eq!(
            html.matches(&hidden).count(),
            3,
            "toggle, rename, delete: {html}"
        );
    }
}
