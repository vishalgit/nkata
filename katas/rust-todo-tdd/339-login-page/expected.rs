use leptos::prelude::*;

use super::UserView;
use crate::error::AppError;

#[server(endpoint = "auth/login")]
pub async fn login(email: String, password: String) -> Result<UserView, AppError> {
    shell::handle(email, password)
        .await
        .map_err(crate::effects::fail)
}

#[cfg(feature = "ssr")]
mod shell {
    use chrono::Utc;
    use domain::identity::{EmailAddress, Role, authenticate};

    use super::*;
    use crate::effects;
    use crate::features::auth::{crypto, store};

    pub async fn handle(email: String, password: String) -> Result<UserView, AppError> {
        let state = effects::state()?;
        let email = EmailAddress::parse(&email).ok();
        let stored = match &email {
            Some(email) => store::find_credentials(&state.pool, email).await?,
            None => None,
        };

        if stored.is_none() {
            crypto::verify_password(&password, &crypto::DUMMY_HASH);
        }

        let actor = authenticate(stored, &password, crypto::verify_password)?;

        effects::start_session(&state, actor.id, Utc::now()).await?;
        Ok(UserView {
            email: email.map(String::from).unwrap_or_default(),
            is_admin: actor.role == Role::Admin,
        })
    }
}

use leptos_router::{components::A, hooks::use_navigate};

use super::context::use_auth;
use crate::ui::ErrorMessage;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    Effect::new(move |_| {
        if let Some(Ok(_)) = auth.login.value().get() {
            navigate("/", Default::default());
        }
    });
    let error = Signal::derive(move || auth.login.value().get().and_then(Result::err));

    view! {
        <section class="auth-card">
            <h1 class="auth-card__title">"Log in"</h1>
            <ActionForm action=auth.login attr:class="form">
                <label class="form__field">
                    <span class="form__label">"Email"</span>
                    <input class="form__input" type="email" name="email" required autocomplete="email"/>
                </label>
                <label class="form__field">
                    <span class="form__label">"Password"</span>
                    <input class="form__input" type="password" name="password" required autocomplete="current-password"/>
                </label>
                <ErrorMessage error/>
                <button class="button button--primary" type="submit">"Log in"</button>
            </ActionForm>
            <p class="auth-card__alt">"No account yet? " <A href="/register">"Register"</A></p>
        </section>
    }
}
