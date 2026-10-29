use domain::todos::Filter;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_query_map;

use super::actions::TodoActions;
use super::create::CreateTodoForm;
use super::list::{Pager, SearchBox, TodoFilters, TodoList, list_todos};
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
    let query = use_query_map();
    let filter_param = move || query.read().get("filter");
    let q_param = move || query.read().get("q").filter(|q| !q.trim().is_empty());
    let page_param = move || query.read().get("page").and_then(|p| p.parse::<u32>().ok());

    let current = Signal::derive(move || Filter::parse(filter_param().as_deref()));
    let q = Signal::derive(q_param);

    let todos = Resource::new(
        move || (filter_param(), q_param(), page_param(), actions.versions()),
        |(filter, q, page, _)| list_todos(filter, q, page),
    );
    let error = Signal::derive(move || actions.error());

    view! {
        <section class="todo-board">
            <h1 class="todo-board__title">"My todos"</h1>
            <CreateTodoForm action=actions.create/>
            <SearchBox filter=current q/>
            <TodoFilters current q/>
            <ErrorMessage error/>
            <Transition fallback=|| view! { <p class="muted">"Loading…"</p> }>
                {move || todos.get().map(|result| match result {
                    Ok(page) if page.items.is_empty() && q.get().is_some() => view! {
                        <p class="todo-list__empty">"No todos match your search."</p>
                    }.into_any(),
                    Ok(page) => view! {
                        <TodoList todos=page.items.clone()/>
                        <Pager page=page.page total_pages=page.total_pages total=page.total
                            filter=current.get_untracked() q=q.get_untracked()/>
                    }.into_any(),
                    Err(e) => view! { <p class="alert alert--error">{e.to_string()}</p> }.into_any(),
                })}
            </Transition>
        </section>
    }
}
