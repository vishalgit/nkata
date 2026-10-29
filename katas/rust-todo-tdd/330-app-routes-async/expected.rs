use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    SsrMode,
    components::{Route, Router, Routes},
    path,
};

use crate::features::auth::context::provide_auth;
use crate::features::todos::page::TodosPage;

/// The HTML document. Rendered on the server only.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// The root component. Runs on the server (SSR) and in the browser (hydrate).
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_auth();

    view! {
        <Stylesheet id="leptos" href="/pkg/todo.css"/>
        <Title text="Todo"/>
        <Router>
            <main class="l-main">
                <Routes fallback=|| view! { <p>"Page not found."</p> }>
                    // Async: wait for me() before sending headers, so the redirect is a real 302
                    <Route path=path!("/") view=TodosPage ssr=SsrMode::Async/>
                </Routes>
            </main>
        </Router>
    }
}
