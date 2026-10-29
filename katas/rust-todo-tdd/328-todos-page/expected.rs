use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::features::auth::context::use_auth;

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
    view! { <h1 class="todo-board__title">"My todos"</h1> }
}
