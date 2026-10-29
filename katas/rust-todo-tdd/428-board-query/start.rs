use leptos::prelude::*;
use leptos_router::components::Redirect;

use super::actions::TodoActions;
use super::create::CreateTodoForm;
use super::list::{TodoList, list_todos};
use crate::features::auth::context::use_auth;
use crate::ui::ErrorMessage;

#[component]
pub fn TodosPage() -> impl IntoView {
    let auth = use_auth();
    view! {
        <Suspense fallback=|| view! { <p class="muted">"Loading…"</p> }>
            {move || auth.user.get().map(|result| match result {
                Ok(Some(_)) => view! { <TodoBoard/> }.into_any(),
                _ => view! { <Redirect path="/login"/> }.into_any(),
            })}
        </Suspense>
    }
}

#[component]
fn TodoBoard() -> impl IntoView {
    let actions = TodoActions::provide();
    let todos = Resource::new(move || actions.versions(), |_| list_todos(None, None, None));
    let error = Signal::derive(move || actions.error());

    view! {
        <section class="todo-board">
            <h1 class="todo-board__title">"My todos"</h1>
            <CreateTodoForm action=actions.create/>
            <ErrorMessage error/>
            <Transition fallback=|| view! { <p class="muted">"Loading…"</p> }>
                {move || todos.get().map(|result| match result {
                    Ok(page) => view! { <TodoList todos=page.items/> }.into_any(),
                    Err(e) => view! { <p class="alert alert--error">{e.to_string()}</p> }.into_any(),
                })}
            </Transition>
        </section>
    }
}
