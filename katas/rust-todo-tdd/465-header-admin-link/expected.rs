use leptos::prelude::*;
use leptos_router::components::A;

use crate::features::auth::context::use_auth;

#[component]
pub fn SiteHeader() -> impl IntoView {
    let auth = use_auth();

    view! {
        <header class="site-header">
            <A href="/" attr:class="site-header__brand">"Todo"</A>
            <nav class="site-header__nav">
                <Transition fallback=|| ()>
                    {move || auth.user.get().map(|result| match result {
                        Ok(Some(user)) => view! {
                            {user.is_admin.then(|| view! { <A href="/admin" attr:class="site-header__link">"Admin"</A> })}
                            <span class="site-header__user">{user.email}</span>
                            <ActionForm action=auth.logout attr:class="site-header__logout">
                                <button class="button button--ghost" type="submit">"Log out"</button>
                            </ActionForm>
                        }.into_any(),
                        _ => view! {
                            <A href="/login" attr:class="site-header__link">"Log in"</A>
                            <A href="/register" attr:class="site-header__link">"Register"</A>
                        }.into_any(),
                    })}
                </Transition>
            </nav>
        </header>
    }
}
